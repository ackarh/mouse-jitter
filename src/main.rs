use enigo::{Enigo, KeyboardControllable, MouseControllable};
use rand::Rng;
use std::{thread, time::Duration};

fn main() {
    let mut enigo = Enigo::new();
    let mut rng = rand::thread_rng();

    println!("Starting anti-idle script...");

    loop {
        // Small random mouse movement
        let dx = rng.gen_range(-2..=2);
        let dy = rng.gen_range(-2..=2);
        enigo.mouse_move_relative(dx, dy);
        println!("Mouse moved by dx: {}, dy: {}", dx, dy);

        // Simulate a harmless key press
        enigo.key_down(enigo::Key::Shift);
        thread::sleep(Duration::from_millis(50));
        enigo.key_up(enigo::Key::Shift);
        println!("Shift key tapped");

        // Wait before next move (adjust delay as needed)
        thread::sleep(Duration::from_secs(30));
    }
}
