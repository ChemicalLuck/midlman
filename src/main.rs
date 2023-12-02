mod midi;
mod volume;

use std::fs::File;
use std::io::stdin;
use std::{error::Error, io::BufReader};

use midir::{Ignore, MidiInput};
use midly::num::u7;
use midly::{live::LiveEvent, MidiMessage};

use midi::{find_in_port, Controller, Preset};
use volume::AudioController;

fn handle_message(stamp: u64, message: &[u8], midi_controller: &mut Controller) {
    let event = LiveEvent::parse(message).unwrap();
    match event {
        LiveEvent::Midi { channel, message } => match message {
            MidiMessage::Controller { controller, value } => {
                println!(
                    "{}: Controller, Channel: {:?} id: {:?} value: {:?}",
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
    // load presets from yaml files
    //
    let subzero_path = "data/subzero.yaml";
    let mut reader = BufReader::new(File::open(subzero_path)?);

    // Select preset
    //
    let subzero_preset: Preset = serde_yaml::from_reader(reader)?;
    println!("{:?}", subzero_preset);

    // load configuration from yaml file
    //

    // create midi controller with preset and configuration
    //
    // run application
    //

    let audio_controller = unsafe { AudioController::new(None) };
    unsafe {
        audio_controller.sessions.iter().for_each(|x| {
            println!("{}", x.get_pid());
            println!("{}", x.get_name());
            println!("{}", x.get_volume());
        });
    }

    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    let in_port = find_in_port(&midi_in)?;

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(&in_port)?;

    // let user select controller from list
    //
    let mut controller = Controller::from_preset(subzero_preset);

    let spotify = unsafe { audio_controller.get_session_by_name("Spotify".to_string()) };
    if let Some(spotify) = spotify {
        println!("Found spotify session");
        controller.bind_component(u7::from(5), spotify);
    }

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
