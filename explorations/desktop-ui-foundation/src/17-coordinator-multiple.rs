use std::thread;
use std::time::Duration;
use crossbeam_channel;
use triple_buffer::TripleBuffer;

// Define all values that state holds in struct
#[derive(Clone, Copy, Debug, PartialEq)]
struct State {
    time: f64,
    resolution: [f64; 2],
    values: [f64; 4],
}

// Create default values when new State struct is created
impl Default for State {
    fn default() -> Self {
        Self { 
            time: 0.0, 
            resolution: [480.0, 320.0], 
            values: [0.5; 4] 
        }
    }
}

// Define messages that are used to update elements in the state
// Note that the SetValue variant is intended to target one element in values
enum Message {
    SetTime(f64),
    SetResolution(f64, f64),
    SetValue(usize, f64),
}

fn main() {
    println!("Coordinator Pattern");

    // Channel for incoming events
    let (sender, receiver) = crossbeam_channel::bounded(5);

    // Triple Buffers for outgoing state
    let (mut window_writer, mut window_reader) = TripleBuffer::new(&State::default()).split();
    let (mut audio_writer, mut audio_reader) = TripleBuffer::new(&State::default()).split();

    // Create Coordinator Thread
    // It owns the receiver and the state_writer
    thread::spawn(move || {
        let mut current_state = State::default();
        let mut last_published_state = current_state;

        while let Ok(update) = receiver.recv() {
            // Update local copy of state
            match update {
                Message::SetTime(t) => current_state.time = t,
                Message::SetResolution(w, h) => current_state.resolution = [w, h],
                Message::SetValue(i, v) => current_state.values[i] = v,
            }

            if current_state != last_published_state {
                // Publish the new state, this is non blocking
                window_writer.write(current_state);
                audio_writer.write(current_state);
                last_published_state = current_state;
                println!("// -> Cooridnator published new state.");
            }
        }
    });

    // Create Midi Thread (Send Only)
    let midi_sender = sender.clone();
    thread::spawn(move || {
        let mut val1 = 0.0;
        let mut val2 = 0.0;
        loop {
            // Use try_send to avoid blocking if channel is full
            midi_sender.try_send(Message::SetValue(0, val1)).ok();
            midi_sender.try_send(Message::SetValue(1, val2)).ok();
            println!("MIDI update value1: {val1} and value2 {val2}");
            val1 += 0.1;
            val2 = val1 * 2.0;
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Create Window Thread (Send and Receive)
    // It owns the state_reader
    let window_sender = sender.clone();
    thread::spawn(move || {
        let mut time = 0.0;
        loop {
            // Send Time Value
            window_sender.try_send(Message::SetTime(time)).ok();
            println!("Window update time: {:.2}", time);

            // Read latest state, this is non blocking
            let state = window_reader.read();
            println!("Window read: {:?}", *state);

            time += 0.1;
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        loop {
            let state = audio_reader.read();
            let freq = state.values[0];
            let amp = state.values[1];
            println!("Audio set freq {:.2} and amp {:.2}", freq, amp);
            thread::sleep(Duration::from_millis(20));
        }
    });

    thread::sleep(Duration::from_secs(1));
}
