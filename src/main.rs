use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let coffee_ascii = r#"
      ( (
       ) )
    ........
    |      |]
    \      /
     `----'
    "#;

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

    println!("{}", coffee_ascii);
    println!("☕️ Coffee Time running...");
    let mut total_minutes = 0;

    loop {
        thread::sleep(Duration::from_secs(300)); // 5 Minutes delay
        total_minutes += 5;

        print!("\x1B[2J\x1B[1;1H"); // Clear screen in each loop

        let hours = total_minutes / 60;
        let mins = total_minutes % 60;

        println!("{}", coffee_ascii);

        // Printing logic using hours and minutes for reading comfort
        if hours > 0 {
            println!(
                "☕️ Drinking coffee for {} hour(s) and {} minutes",
                hours, mins
            );
        } else {
            println!("☕️ Drinking coffee for {} minutes", mins);
        }

        // Move one pixel right and one pixel down
        let _ = enigo.move_mouse(1, 1, Coordinate::Rel);

        thread::sleep(Duration::from_millis(100));

        // Return to original position
        let _ = enigo.move_mouse(-1, -1, Coordinate::Rel);
    }
}
