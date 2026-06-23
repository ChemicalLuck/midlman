# midlman

A MIDI-to-Windows-audio-mixer bridge written in Rust. `midlman` reads input from a MIDI controller (sliders, dials, and buttons) and uses it to control per-application volume and mute on Windows, via the Core Audio (WASAPI) session API.

Think of it as a hardware mixing desk for your Windows audio sessions, controlled from any class-compliant MIDI device.

## How it works

1. `midlman` lists the active Windows audio sessions (one per running application with audio output) along with their current volume.
2. It opens a connection to your MIDI controller and waits for input.
3. A **preset** (YAML) describes which MIDI CC numbers on your controller correspond to sliders, dials, and buttons.
4. Component bindings map specific CC numbers to specific audio sessions, for example slider 5 to Spotify's volume, button 25 to Spotify's mute.
5. Moving a slider/dial or pressing a button sends a MIDI Control Change message, which `midlman` translates into a volume or mute call against the matching session.
6. SysEx messages are used to switch **banks**, so a single controller can address more application bindings than it has physical components.

## Requirements

- Windows (the audio backend uses the Win32 Core Audio APIs directly)
- Rust toolchain (`cargo`)
- A class-compliant MIDI controller with sliders, dials, and/or buttons

## Building

```bash
git clone https://github.com/ChemicalLuck/midlman.git
cd midlman
cargo build --release
```

The binary is built as `midlman` (see `[[bin]]` in `Cargo.toml`).

## Presets

A preset declares which CC numbers belong to each component type, as comma-separated values and ranges:

```yaml
components:
  sliders: "3-11"
  buttons: "1,2,23-31,44-49,64,67"
  dials: "12-22"
```

`midlman` currently loads a preset from a hardcoded path, `data/subzero.yaml`, at startup. You'll need to create a `data/` directory in the project root and add your controller's preset there before running.

## Bindings

Component-to-session bindings (which CC controls which application) are currently wired up in code rather than config, see the `Spotify` example in `src/main.rs`:

```rust
controller.bind_component(
    u7::from(5),
    Box::new(move |value| {
        let normalized = value.as_int() as f32 / 127.0;
        spotify_session.lock().unwrap().set_volume(normalized);
    }),
);
```

To target a different application, change the session name passed to `get_session_by_name` and the CC numbers passed to `bind_component`.

## Running

```bash
cargo run --release
```

On startup, `midlman` will:

- Print all active audio sessions and their current volume
- Prompt you to select a MIDI input port if more than one is connected
- Begin listening for MIDI input until you press Enter to exit

Pass `--debug` to print every incoming Control Change and SysEx message as it arrives, useful for finding the CC numbers your controller sends:

```bash
cargo run --release -- --debug
```

## Project layout

```
src/
├── main.rs              # Entry point, session discovery, example bindings
├── midi/
│   ├── mod.rs            # Port selection
│   ├── controller.rs     # Controller — bank-aware component lookup and dispatch
│   ├── preset.rs         # Preset / PresetComponents — YAML schema
│   └── components/       # Slider, Dial, Button component types
└── volume/
    ├── mod.rs
    ├── session.rs         # Session — per-application volume/mute wrapper
    └── winaudio.rs        # AudioController — Win32 Core Audio session enumeration
```

## License

No license file is currently included in this repository.
