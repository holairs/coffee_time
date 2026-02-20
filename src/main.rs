use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use terminal_size::{Height, Width, terminal_size};

fn main() {
    // Array of sprites for the animation
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

    // Start caffeinate in an independent thread
    thread::spawn(|| {
        let mut child = Command::new("caffeinate")
            .arg("-ids")
            .spawn()
            .expect("Error while trying to run caffeinate (macOS)");

        let _ = child.wait();
    });

    // Initialize Enigo with default settings
    let mut enigo = Enigo::new(&Settings::default()).expect("Error while trying to run Enigo");

    let mut total_seconds = 0;
    let mut last_mouse_move = Instant::now();
    let mut frame = 0;

    loop {
        // Clear screen in each loop
        print!("\x1B[2J\x1B[1;1H");

        // Get terminal size for centering
        let (Width(w), Height(h)) = terminal_size().unwrap_or((Width(80), Height(24)));

        // Calculate vertical padding
        let sprite = coffee_sprites[frame % coffee_sprites.len()];
        let sprite_lines: Vec<&str> = sprite.lines().filter(|l| !l.is_empty()).collect();
        // We add +2 to the sprite height to account for the message below
        let vertical_start = (h as usize).saturating_sub(sprite_lines.len() + 2) / 2;

        for _ in 0..vertical_start {
            println!();
        }

        // Center and print the sprite
        for line in &sprite_lines {
            // Using chars().count() for centering
            let padding = (w as usize).saturating_sub(line.chars().count()) / 2;
            println!("{}{}", " ".repeat(padding), line);
        }

        // Center and print the text
        let hours = total_seconds / 3600;
        let mins = (total_seconds % 3600) / 60;

        let msg = if hours > 0 {
            format!(
                "☕️ Drinking coffee for {} hour(s) and {} minutes",
                hours, mins
            )
        } else {
            format!("☕️ Drinking coffee for {} minutes", mins)
        };

        let text_padding = (w as usize).saturating_sub(msg.chars().count()) / 2;
        println!("{}{}", " ".repeat(text_padding), msg);

        // Animation logic: Change frame every second
        frame += 1;
        thread::sleep(Duration::from_secs(1));
        total_seconds += 1;

        // Mouse movement every 5 minutes (300 seconds)
        if last_mouse_move.elapsed() >= Duration::from_secs(300) {
            // Move one pixel right and one pixel down
            let _ = enigo.move_mouse(1, 1, Coordinate::Rel);
            thread::sleep(Duration::from_millis(100));
            // Return to original position
            let _ = enigo.move_mouse(-1, -1, Coordinate::Rel);
            last_mouse_move = Instant::now();
        }
    }
}
