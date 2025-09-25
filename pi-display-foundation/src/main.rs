
mod modules;
#[cfg(target_os = "linux")]
use modules::display::*;

fn main() {
    #[cfg(target_os = "linux")]
    start_display();

    println!("Hello people.");
}
