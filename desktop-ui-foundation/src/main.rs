use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
struct SimpleState {
    value: f64,
}

fn main() {
    println!("Basic Shared State");

    // Arc (Atomic Reference Counter) allows multiple owners
    // Mutex (Mutual Exclusion) only one thread can access at the same time
    let shared_state = Arc::new(Mutex::new(SimpleState{ value: 0.0}));

    // Producer Thread
    let producer_state = shared_state.clone();
    let producer = thread::spawn(move || {
        for i in 0..5 {
            let mut state = producer_state.lock().unwrap();
            state.value = i as f64;
            println!("Producer set value to: {}", state.value);
            drop(state);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let consumer_state = shared_state.clone();
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            let state = consumer_state.lock().unwrap();
            println!("Consumer read value: {}", state.value);
            drop(state);
            thread::sleep(Duration::from_millis(150));
        }
    });

    // The .join() method will pause the current thread (in this case main) 
    // until the spawned thread completes execution
    // When you do not use that the thread is spawned, but the main is already done and drops it
    producer.join().unwrap();
    consumer.join().unwrap();
}
