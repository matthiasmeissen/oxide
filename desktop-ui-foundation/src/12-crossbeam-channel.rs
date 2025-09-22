use std::thread;
use std::time::Duration;
use crossbeam_channel;

// This is the messages we can send through the channel
#[derive(Debug)]
enum Message {
    UpdateValue(f64),
    Print(String),
}

fn main() {
    println!("Channel communication");


    let (sender, receiver) = crossbeam_channel::bounded(5);

    // Producer 1
    let sender1 = sender.clone();
    thread::spawn(move || {
        for i in 0..3 {
            sender1.send(Message::UpdateValue(i as f64)).unwrap();
            println!("Update value");
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Producer 2
    let sender2 = sender.clone();
    thread::spawn(move || {
        for i in 0..3 {
            sender2.send(Message::Print(format!("Hello {}", i))).unwrap();
            println!("Update string");
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Consumer
    thread::spawn(move || {
        // Does this variable stay alive for the whole existence of the thread
        let mut current_value = 0.0;

        // Is this a loop that runs over and over and looks for messages
        while let Ok(msg) = receiver.recv() {
            match msg {
                Message::UpdateValue(v) => {
                    current_value = v;
                    println!("Updated value to {}", current_value);
                },
                Message::Print(s) => {
                    println!("Print message: {} , current value: {}", s, current_value);
                }
            }
        }
    });

    // This keeps the main thread and all other threads alive
    thread::sleep(Duration::from_secs(1));
}
