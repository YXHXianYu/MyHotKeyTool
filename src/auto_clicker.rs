use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::input_control::InputController;
use rdev::{listen, EventType, Key};

pub struct AutoClicker {
    is_left_clicking: Arc<AtomicBool>,
    is_right_clicking: Arc<AtomicBool>,
    click_thread: Option<thread::JoinHandle<()>>,
}

impl AutoClicker {
    pub fn new() -> Self {
        Self {
            is_left_clicking: Arc::new(AtomicBool::new(false)),
            is_right_clicking: Arc::new(AtomicBool::new(false)),
            click_thread: None,
        }
    }

    pub fn start_listener(mut self) {
        let is_left_clicking = self.is_left_clicking.clone();
        let is_right_clicking = self.is_right_clicking.clone();
        let mut ctrl_pressed = false;
        let mut shift_pressed = false;
        
        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    match key {
                        Key::ControlLeft | Key::ControlRight => ctrl_pressed = true,
                        Key::ShiftLeft | Key::ShiftRight => shift_pressed = true,
                        Key::KeyJ if ctrl_pressed && shift_pressed => {
                            let running = !is_left_clicking.load(Ordering::SeqCst);
                            is_left_clicking.store(running, Ordering::SeqCst);
                            
                            if running {
                                self.start_clicking_left();
                            }
                            println!("Auto clicking [Left]: {}", running);
                        },
                        Key::KeyK if ctrl_pressed && shift_pressed => {
                            let running = !is_right_clicking.load(Ordering::SeqCst);
                            is_right_clicking.store(running, Ordering::SeqCst);
                            
                            if running {
                                self.start_clicking_right();
                            }
                            println!("Auto clicking [Right]: {}", running);
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

    fn start_clicking_left(&mut self) {
        let is_running = self.is_left_clicking.clone();
        
        self.click_thread = Some(thread::spawn(move || {
            let mut controller = InputController::new();
            
            while is_running.load(Ordering::SeqCst) {
                controller.click_left_button();
                thread::sleep(Duration::from_millis(50));
            }
        }));
    }

    fn start_clicking_right(&mut self) {
        let is_running = self.is_right_clicking.clone();
        
        self.click_thread = Some(thread::spawn(move || {
            let mut controller = InputController::new();
            
            while is_running.load(Ordering::SeqCst) {
                controller.click_right_button();
                thread::sleep(Duration::from_millis(50));
            }
        }));
    }
} 