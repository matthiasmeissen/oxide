use std::thread;
use std::time::Duration;
use ringbuf;
use ringbuf::traits::{Consumer, Producer, Split};

fn main() {
    println!("Ringbuffer Data Streaming");

    let rb = ringbuf::HeapRb::<f64>::new(2);
    let (mut producer, mut consumer) = rb.split();

    thread::spawn(move || {
        let mut num = 0.0;
        loop {
            match producer.try_push(num) {
                Ok(_) => println!("// -> Producer sent value: {}", num),
                Err(_) => println!("// -> Producer could not send, buffer is full."),
            }
            num += 1.0;
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        loop {
            match consumer.try_pop() {
                Some(v) => println!("Consumer read value: {}", v),
                None => println!("Could not read value, buffer is empty."),
            }
            thread::sleep(Duration::from_millis(20));
        }
    });

    thread::sleep(Duration::from_secs(1));

}
