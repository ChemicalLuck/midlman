use std::process::exit;

use windows::{
    core::GUID,
    Win32::{
        Foundation::BOOL,
        Media::Audio::{Endpoints::IAudioEndpointVolume, ISimpleAudioVolume},
    },
};

fn linear_to_logarithmic(vol: f32) -> f32 {
    let vol = vol.clamp(0.0, 1.0);
    2.0f32.powf(vol.powf(4.0)) - 1.0
}

pub trait Session: Send {
    unsafe fn get_audio_endpoint_volume(&self) -> Option<IAudioEndpointVolume>;
    unsafe fn get_name(&self) -> String;
    unsafe fn get_pid(&self) -> u32;
    unsafe fn get_volume(&self) -> f32;
    unsafe fn set_volume(&self, vol: f32);
    unsafe fn get_mute(&self) -> bool;
    unsafe fn set_mute(&self, mute: bool);
    fn clone_dyn(&self) -> Box<dyn Session>;
}

impl Clone for Box<dyn Session> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

#[derive(Clone)]
pub struct EndPointSession {
    simple_audio_volume: IAudioEndpointVolume,
    name: String,
    pid: u32,
    guid: GUID,
}

impl EndPointSession {
    pub fn new(simple_audio_volume: IAudioEndpointVolume, name: String, pid: u32) -> Self {
        let guid = GUID::new().unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't generate GUID {err}");
            exit(1);
        });

        Self {
            simple_audio_volume,
            name,
            pid,
            guid,
        }
    }
}

unsafe impl Send for EndPointSession {}

impl Session for EndPointSession {
    unsafe fn get_audio_endpoint_volume(&self) -> Option<IAudioEndpointVolume> {
        Some(self.simple_audio_volume.clone())
    }

    unsafe fn get_name(&self) -> String {
        self.name.clone()
    }

    unsafe fn get_pid(&self) -> u32 {
        self.pid
    }

    unsafe fn get_volume(&self) -> f32 {
        self.simple_audio_volume
            .GetMasterVolumeLevelScalar()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get volume {err}");
                0.0
            })
    }
    unsafe fn set_volume(&self, vol: f32) {
        let vol = linear_to_logarithmic(vol);
        self.simple_audio_volume
            .SetMasterVolumeLevelScalar(vol, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set volume: {err}");
            });
    }
    unsafe fn set_mute(&self, mute: bool) {
        self.simple_audio_volume
            .SetMute(mute, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set mute: {err}");
            });
    }
    unsafe fn get_mute(&self) -> bool {
        self.simple_audio_volume
            .GetMute()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get mute {err}");
                BOOL(0)
            })
            .as_bool()
    }

    fn clone_dyn(&self) -> Box<dyn Session> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct ApplicationSession {
    simple_audio_volume: ISimpleAudioVolume,
    name: String,
    pid: u32,
    guid: GUID,
}

impl ApplicationSession {
    pub fn new(simple_audio_volume: ISimpleAudioVolume, name: String, pid: u32) -> Self {
        let guid = GUID::new().unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't generate GUID {err}");
            exit(1);
        });

        Self {
            simple_audio_volume,
            name,
            pid,
            guid,
        }
    }
}

unsafe impl Send for ApplicationSession {}

impl Session for ApplicationSession {
    unsafe fn get_audio_endpoint_volume(&self) -> Option<IAudioEndpointVolume> {
        None
    }

    unsafe fn get_name(&self) -> String {
        self.name.clone()
    }

    unsafe fn get_pid(&self) -> u32 {
        self.pid
    }

    unsafe fn get_volume(&self) -> f32 {
        self.simple_audio_volume
            .GetMasterVolume()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get volume {err}");
                0.0
            })
    }
    unsafe fn set_volume(&self, vol: f32) {
        let vol = linear_to_logarithmic(vol);
        self.simple_audio_volume
            .SetMasterVolume(vol, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set volume: {err}");
            });
    }
    unsafe fn set_mute(&self, mute: bool) {
        self.simple_audio_volume
            .SetMute(mute, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set mute: {err}");
            });
    }
    unsafe fn get_mute(&self) -> bool {
        self.simple_audio_volume
            .GetMute()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get mute {err}");
                BOOL(0)
            })
            .as_bool()
    }
    fn clone_dyn(&self) -> Box<dyn Session> {
        Box::new(self.clone())
    }
}
