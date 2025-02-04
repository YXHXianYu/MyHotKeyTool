mod input_control;
mod auto_clicker;

use auto_clicker::AutoClicker;

fn main() {
    let auto_clicker = AutoClicker::new();
    auto_clicker.start_listener();
}