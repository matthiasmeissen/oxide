use std::thread;
use std::time::Duration;
use triple_buffer::TripleBuffer;

#[derive(Clone, Copy, Debug)]
struct AudioState {
    freq: f64,
    amp: f64,
}

impl Default for AudioState {
    fn default() -> Self {
        Self { freq: 440.0, amp: 0.4 }
    }
}

fn main() {
    println!("Triple Buffer Lock Free");

    let (mut writer, mut reader) = TripleBuffer::new(&AudioState::default()).split();

    thread::spawn(move || {
        let mut num = 0.0;
        loop {
            let state = AudioState {
                freq: 440.0 + (num),
                amp: num.sin(),
            };
            writer.write(state);
            println!("// -> Wrote {:?}", state);
            num += 1.0;
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        loop {
            let state = reader.read();
            println!("Read: {:?}", state);
            thread::sleep(Duration::from_millis(20));
        }
    });

    thread::sleep(Duration::from_secs(1));

}
