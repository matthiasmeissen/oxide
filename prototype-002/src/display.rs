#![cfg(target_os = "linux")]

use crate::{state::*, screens::*};

use sh1106::{prelude::*, Builder};
use linux_embedded_hal::I2cdev;

use triple_buffer::*;

use std::thread;
use std::time::Duration;

pub fn start_display(mut display_reader: Output<State>) {
    thread::spawn(move || {
        let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
        i2c.set_slave_address(0x3C).unwrap();
    
        let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    
        display.init().unwrap();
        display.flush().unwrap();
    
        loop {
            let state = display_reader.read();
    
            display.clear();
    
            match state.screen_index {
                0 => draw_screen(&mut display, &state),
                _ => draw_debug(&mut display, &state),
            }
        
            display.flush().unwrap();
    
            thread::sleep(Duration::from_millis(40));
        }
    });
}
