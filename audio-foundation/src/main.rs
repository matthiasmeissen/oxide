
use std::io::stdin;
use midir::{Ignore, MidiInput};

fn main() {
    println!("Midi Input");

    let mut midi_in = MidiInput::new("Midi Input 1").unwrap();
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let port = match in_ports.len() {
        0 => {
            println!("No midi port available.");
            return;
        },
        1 => {
            println!("Connecting to port {}", midi_in.port_name(&in_ports[0]).unwrap());
            &in_ports[0]
        },
        _ => {
            println!("Too many ports.");
            return;
        },
    };

    let connection = midi_in.connect(
        port, "Midi Input", |timestamp, message, _| {
            println!("{}: {:?}", timestamp, message);
        }, ()).unwrap();

    println!("Listening, press a key to quit program.");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
