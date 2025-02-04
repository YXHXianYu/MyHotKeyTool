use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::input_control::InputController;
use rdev::{listen, EventType, Key};

pub struct AutoClicker {
    is_running: Arc<AtomicBool>,
    click_thread: Option<thread::JoinHandle<()>>,
}

impl AutoClicker {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            click_thread: None,
        }
    }

    pub fn start_listener(mut self) {
        let is_running = self.is_running.clone();
        let mut ctrl_pressed = false;
        let mut shift_pressed = false;
        
        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    match key {
                        Key::ControlLeft | Key::ControlRight => ctrl_pressed = true,
                        Key::ShiftLeft | Key::ShiftRight => shift_pressed = true,
                        Key::KeyJ if ctrl_pressed && shift_pressed => {
                            let running = !is_running.load(Ordering::SeqCst);
                            is_running.store(running, Ordering::SeqCst);
                            
                            if running {
                                self.start_clicking();
                            }
                            println!("Auto clicking: {}", running);
                        },
                        Key::KeyK if ctrl_pressed && shift_pressed => {
                            is_running.store(false, Ordering::SeqCst);
                            println!("Auto clicking: false");
                        },
                        _ => {}
                    }
                },
                EventType::KeyRelease(key) => {
                    match key {
                        Key::ControlLeft | Key::ControlRight => ctrl_pressed = false,
                        Key::ShiftLeft | Key::ShiftRight => shift_pressed = false,
                        _ => {}
                    }
                },
                _ => {}
            }
        }) {
            eprintln!("Error: {:?}", error);
        }
    }

    fn start_clicking(&mut self) {
        let is_running = self.is_running.clone();
        
        self.click_thread = Some(thread::spawn(move || {
            let mut controller = InputController::new();
            
            while is_running.load(Ordering::SeqCst) {
                controller.click_left_button();
                thread::sleep(Duration::from_millis(50));
            }
        }));
    }
} 