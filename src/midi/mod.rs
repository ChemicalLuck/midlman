mod components;
mod controller;
mod preset;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{MidiInput, MidiInputPort};

// re-export
pub use controller::Controller;
pub use preset::Preset;

pub fn find_in_port(midi_in: &MidiInput) -> Result<MidiInputPort, Box<dyn Error>> {
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            in_ports[0].clone()
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .map(|p| p.clone())
                .ok_or("invalid input port selected")?
        }
    };
    Ok(in_port)
}
