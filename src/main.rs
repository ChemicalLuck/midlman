mod component;
mod controller;
mod midi;

use std::error::Error;
use std::io::stdin;

use midir::{Ignore, MidiInput};
use midly::{live::LiveEvent, MidiMessage};

use controller::Controller;
use midi::find_in_port;

fn handle_message(stamp: u64, message: &[u8], midi_controller: &mut Controller) {
    let event = LiveEvent::parse(message).unwrap();
    match event {
        LiveEvent::Midi { channel, message } => match message {
            MidiMessage::Controller { controller, value } => {
                println!(
                    "{}: Controller {:?} {:?} {:?}",
                    stamp, channel, controller, value
                );
                midi_controller.set_component(controller, value);
            }
            _ => todo!(),
        },
        LiveEvent::Common(sys_common) => match sys_common {
            midly::live::SystemCommon::SysEx(bytes) => {
                println!("{}: SysEx {:?}", stamp, bytes);
                midi_controller.set_bank(bytes[bytes.len() - 1]);
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    let in_port = find_in_port(&midi_in)?;

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(&in_port)?;

    // let user select controller from list
    //
    let mut controller = Controller::new();

    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| handle_message(stamp, message, &mut controller),
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
