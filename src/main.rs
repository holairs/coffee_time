use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::process::Child;

#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::process::{Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};
use terminal_size::{Height, Width, terminal_size};

#[cfg(windows)]
use windows_sys::Win32::System::Power::{
    ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, SetThreadExecutionState,
};

struct KeepAwake {
    child: Option<Child>,
}

impl KeepAwake {
    fn start() -> Self {
        #[cfg(target_os = "macos")]
        {
            let child = Command::new("caffeinate")
                .args(["-d", "-i", "-s"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("No se pudo iniciar caffeinate en macOS");

            return Self { child: Some(child) };
        }

        #[cfg(target_os = "linux")]
        {
            let child = Command::new("systemd-inhibit")
                .args([
                    "--what=idle:sleep",
                    "--who=coffe_time",
                    "--why=Keeping the system awake",
                    "sleep",
                    "infinity",
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .ok();

            if child.is_none() {
                eprintln!(
                    "Advertencia: no se pudo iniciar systemd-inhibit. \
                     Se continuará solo con movimiento del mouse."
                );
            }

            return Self { child };
        }

        #[cfg(windows)]
        {
            unsafe {
                let result = SetThreadExecutionState(
                    ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED,
                );

                if result == 0 {
                    eprintln!(
                        "Advertencia: SetThreadExecutionState falló. \
                         Se continuará solo con movimiento del mouse."
                    );
                }
            }

            return Self { child: None };
        }

        #[allow(unreachable_code)]
        Self { child: None }
    }
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.kill();
            let _ = child.wait();
        }

        #[cfg(windows)]
        unsafe {
            let _ = SetThreadExecutionState(ES_CONTINUOUS);
        }
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_handler = Arc::clone(&running);

    ctrlc::set_handler(move || {
        running_handler.store(false, Ordering::SeqCst);
    })
    .expect("No se pudo configurar el manejador de Ctrl+C");

    let _keep_awake = KeepAwake::start();

    let coffee_sprites = [
        r#"
( (
 ) )
........
|      |]
\      /
`----'
~"#,
        r#"
 ) )
( (
........
|      |]
\      /
`----'
~~"#,
    ];

    let mut enigo = Enigo::new(&Settings::default())
        .expect("No se pudo inicializar Enigo");

    let mut total_seconds = 0;
    let mut last_mouse_move = Instant::now();
    let mut frame = 0;

    while running.load(Ordering::SeqCst) {
        print!("\x1B[2J\x1B[1;1H");

        let (Width(w), Height(h)) = terminal_size().unwrap_or((Width(80), Height(24)));

        let sprite = coffee_sprites[frame % coffee_sprites.len()];
        let sprite_lines: Vec<&str> = sprite.lines().filter(|l| !l.is_empty()).collect();

        let vertical_start = (h as usize).saturating_sub(sprite_lines.len() + 2) / 2;

        for _ in 0..vertical_start {
            println!();
        }

        for line in &sprite_lines {
            let padding = (w as usize).saturating_sub(line.chars().count()) / 2;
            println!("{}{}", " ".repeat(padding), line);
        }

        let hours = total_seconds / 3600;
        let mins = (total_seconds % 3600) / 60;

        let msg = if hours > 0 {
            format!(
                "Drinking coffee for {} hour(s) and {} minutes",
                hours, mins
            )
        } else {
            format!("Drinking coffee for {} minutes", mins)
        };

        let text_padding = (w as usize).saturating_sub(msg.chars().count()) / 2;
        println!("{}{}", " ".repeat(text_padding), msg);

        frame += 1;
        thread::sleep(Duration::from_secs(1));
        total_seconds += 1;

        if last_mouse_move.elapsed() >= Duration::from_secs(300) {
            let _ = enigo.move_mouse(1, 1, Coordinate::Rel);
            thread::sleep(Duration::from_millis(100));
            let _ = enigo.move_mouse(-1, -1, Coordinate::Rel);
            last_mouse_move = Instant::now();
        }
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("coffe_time finalizado correctamente.");
}
