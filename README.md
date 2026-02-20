# Coffee Time ☕️

**Coffee Time** is a lightweight macOS utility written in Rust. It ensures your Mac stays awake during long-running tasks, downloads, or presentations by combining the native `caffeinate` command with subtle hardware-level interaction.

---

## 🚀 Features

* **System Wake-lock:** Executes `caffeinate -ids` to prevent system sleep, disk idle, and display dimming.
* **Active Presence:** Moves the mouse cursor by 1 pixel every 5 minutes to simulate user activity (bypassing stricter IT policies).
* **Minimalist Dashboard:** Displays a clean ASCII coffee mug and a real-time counter of how long your system has been "caffeinated."
* **Automatic Cleanup:** When you stop the program, it safely terminates the background processes.

---

## 📸 Preview

<img width="904" height="426" alt="image" src="https://github.com/user-attachments/assets/f49e2519-89ba-46ff-b791-0caac20313e7" />


*The terminal displays a clean ASCII coffee mug and the elapsed time.*

---

## 🛠 Installation & Setup

### 1. Project Configuration
Ensure your `Cargo.toml` is configured as follows:

```toml
[package]
name = "coffee_time"
version = "0.1.0"
edition = "2024"

[dependencies]
enigo = "0.6.1"
```


### 2. Build & Run

```Bash
cargo run --release
```

### Run it from Anywhere on Your Mac

If you want to build the release binary **and** make it available system-wide in a single command, run:

```bash
cargo build --release && sudo cp target/release/coffee_time /usr/local/bin/coffee_time
```

### 3. Permissions (Important!)
Since this app controls the mouse cursor, macOS requires Accessibility Permissions:

* When prompted, open System Settings.
* Go to Privacy & Security > Accessibility.
* Toggle the switch ON for your Terminal (e.g., iTerm2, Terminal, or VS Code).

## 📖 How it Works
The app operates in two layers:

The Process: It spawns a child process running caffeinate. This is the official Apple-sanctioned way to prevent sleep at the kernel level.

The Loop: Every 5 minutes, it moves the mouse relative to its current position by 1 pixel and immediately moves it back. This resets idle timers without interrupting your work or being visible to the eye.

Stopping the App
Simply press Ctrl + C in your terminal. This will stop the counter and kill the caffeinate process automatically.

### Stay caffeinated! ☕️
