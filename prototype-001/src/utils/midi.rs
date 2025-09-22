use crate::utils::state::*;

use std::thread;
use std::io::*;
use crossbeam_channel::Sender;
use midir::*;

pub fn start_midi_thread(midi_sender: Sender<Message>) {
    thread::spawn(move || {
        let mut midi_in = MidiInput::new("Midi Input 1").unwrap();
        midi_in.ignore(Ignore::None);

        let in_ports = midi_in.ports();

        if in_ports.is_empty() {
            println!("No midi port available.");
            return;
        }

        let target_port = in_ports.iter().find(|p| {
            midi_in.port_name(p).unwrap_or_default().contains("OP-Z")
        });

        let port = match target_port {
            Some(p) => {
                println!("Automatically connected to port: {}", midi_in.port_name(p).unwrap());
                p
            },
            None => {
                println!("\nCould not use automatic port. Available input ports:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_in.port_name(p).unwrap());
                }
                print!("Please select input port: ");
                stdout().flush().unwrap();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                in_ports
                    .get(input.trim().parse::<usize>().unwrap())
                    .ok_or("invalid input port selected").unwrap()
            }
        };

        println!("\nOpening connection to {}", midi_in.port_name(port).unwrap());
        let _connection = midi_in.connect(
            port, "Midi Input", move |_timestamp, message, _| {
                if let Some(parsed_message) = parse_midi_message(message) {
                    println!("MIDI parsed: {:?}", parsed_message);
                    midi_sender.try_send(Message::MidiInput(parsed_message)).ok();
                }
            }, ()
        ).unwrap();

        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

fn parse_midi_message(message: &[u8]) -> Option<MidiMessage> {
    if message.len() < 3 {
        return None;
    }

    let command = message[0] & 0xF0;

    match command {
        // 0x90 = Note On
        0x90 => {
            let note = message[1];
            let velocity = message[2];
            // A Note On with velocity 0 is often treated as a Note Off
            if velocity > 0 {
                Some(MidiMessage::NoteOn { note, velocity })
            } else {
                Some(MidiMessage::NoteOff { note })
            }
        }
        // 0x80 = Note Off
        0x80 => {
            let note = message[1];
            Some(MidiMessage::NoteOff { note })
        }
        // 0xB0 = Control Change (CC)
        0xB0 => {
            let controller = message[1];
            let value = message[2];
            Some(MidiMessage::ControlChange { controller, value })
        }
        _ => None,
    }
}
