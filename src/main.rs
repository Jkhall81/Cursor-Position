use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use std::io::{self, Read};
use std::time::{Instant, Duration};

fn main() {
    println!("Press Enter to start listening for mouse clicks...");
    wait_for_enter();

    println!("Mouse click listener started. Press Enter to pause, Ctrl+C to exit.");
    println!("Waiting for mouse clicks...");

    let device_state = DeviceState::new();
    let mut previous_mouse = MouseState::default();
    let mut paused = false;
    let mut last_enter_time = Instant::now();

    // Add a small delay after startup to ignore the initial Enter press
    std::thread::sleep(Duration::from_millis(500));

    loop {
        // Check for Enter key press to toggle pause
        if let Some(true) = check_enter_pressed(&device_state) {
            // Only register the Enter press if it's been at least 500ms since the last one
            if last_enter_time.elapsed() > Duration::from_millis(500) {
                paused = !paused;
                if paused {
                    println!("Program paused. Press Enter to resume...");
                } else {
                    println!("Resuming...");
                }
                last_enter_time = Instant::now();
            }
            // Wait for Enter to be released
            while check_enter_pressed(&device_state).unwrap_or(false) {
                std::thread::sleep(Duration::from_millis(50));
            }
        }

        if paused {
            std::thread::sleep(Duration::from_millis(100));
            continue;
        }

        let mouse = device_state.get_mouse();
        
        // Check if any button was pressed
        if mouse.button_pressed.iter().any(|&pressed| pressed) && 
           previous_mouse.button_pressed.iter().any(|&pressed| !pressed) {
            println!("Clicked at: x={}, y={}", mouse.coords.0, mouse.coords.1);
        }
        
        previous_mouse = mouse;
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn wait_for_enter() {
    let mut buffer = [0; 1];
    while io::stdin().read(&mut buffer).is_ok() {
        if buffer[0] == b'\n' {
            break;
        }
    }
}

fn check_enter_pressed(device_state: &DeviceState) -> Option<bool> {
    let keys = device_state.get_keys();
    Some(keys.contains(&Keycode::Enter))
}