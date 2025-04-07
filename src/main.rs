mod input_control;
mod auto_clicker;

use auto_clicker::AutoClicker;

fn main() {
    println!("Press `Ctrl+Shift+J` to start auto-clicking Left Button");
    println!("Press `Ctrl+Shift+R` to start auto-clicking Right Button");
    let auto_clicker = AutoClicker::new();
    auto_clicker.start_listener();
}