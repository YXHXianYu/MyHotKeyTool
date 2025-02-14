mod input_control;
mod auto_clicker;

use auto_clicker::AutoClicker;

fn main() {
    println!("Press `Ctrl+Shift+J` to start auto-clicking");
    println!("Press again or press `Ctrl+Shift+K` to stop auto-clicking");
    let auto_clicker = AutoClicker::new();
    auto_clicker.start_listener();
}