use device_query::{DeviceQuery, DeviceState, MouseState};

fn main() {
    println!("Mouse click listener started. Press Ctrl+C to exit.");
    println!("Waiting for mouse clicks...");

    let device_state = DeviceState::new();
    let mut previous_mouse = MouseState::default();

    loop {
        let mouse = device_state.get_mouse();
        
        // Check if any button was pressed
        if mouse.button_pressed.iter().any(|&pressed| pressed) && 
           previous_mouse.button_pressed.iter().any(|&pressed| !pressed) {
            println!("Clicked at: x={}, y={}", mouse.coords.0, mouse.coords.1);
        }
        
        previous_mouse = mouse;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}