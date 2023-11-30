use std::collections::HashSet;
use std::process::exit;
use windows::core::ComInterface;
use windows::Win32::{
    Media::Audio::{
        eRender, IAudioSessionControl, IAudioSessionControl2, IAudioSessionManager2, IMMDevice,
        IMMDeviceCollection, IMMDeviceEnumerator, ISimpleAudioVolume, MMDeviceEnumerator,
        DEVICE_STATE_ACTIVE,
    },
    System::{
        Com::{
            CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
            COINIT_MULTITHREADED,
        },
        ProcessStatus::K32GetProcessImageFileNameA,
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

use crate::volume::session::{ApplicationSession, Session};

pub struct AudioController {
    pub sessions: Vec<Box<dyn Session>>,
}

pub enum CoInitMode {
    MultiThreaded,
    ApartmentThreaded,
}

impl AudioController {
    pub unsafe fn new(coinit_mode: Option<CoInitMode>) -> Self {
        let mut coinit: windows::Win32::System::Com::COINIT = COINIT_MULTITHREADED;
        if let Some(x) = coinit_mode {
            match x {
                CoInitMode::MultiThreaded => coinit = COINIT_MULTITHREADED,
                CoInitMode::ApartmentThreaded => coinit = COINIT_APARTMENTTHREADED,
            }
        }

        CoInitializeEx(None, coinit).unwrap_or_else(|err| {
            eprintln!("ERRORL: Couldn't initialize windows connection: {err}");
            exit(1);
        });

        let sessions = Self::get_sessions();

        Self { sessions }
    }

    unsafe fn get_imm_device_enumerator() -> IMMDeviceEnumerator {
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER).unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't create instance of MMDeviceEnumerator: {err}");
            exit(1);
        })
    }

    unsafe fn get_imm_device_collection() -> IMMDeviceCollection {
        let enumerator = Self::get_imm_device_enumerator();
        enumerator
            .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't enumerate audio endpoints: {err}");
                exit(1);
            })
    }

    unsafe fn get_devices() -> Vec<IMMDevice> {
        let devices_collection = Self::get_imm_device_collection();

        let mut devices = Vec::new();
        for i in 0..devices_collection.GetCount().unwrap() {
            devices.push(devices_collection.Item(i).unwrap());
        }
        devices
    }

    // pub unsafe fn get_default_audio_enpoint_volume_control(&mut self) {
    //     if self.imm_device_enumerator.is_none() {
    //         eprintln!("ERROR: Function called before creating enumerator");
    //         return;
    //     }
    //
    //     self.default_device = Some(
    //         self.imm_device_enumerator
    //             .clone()
    //             .unwrap()
    //             .GetDefaultAudioEndpoint(eRender, eMultimedia)
    //             .unwrap_or_else(|err| {
    //                 eprintln!("ERROR: Couldn't get Default audio endpoint {err}");
    //                 exit(1);
    //             }),
    //     );
    //     let simple_audio_volume: IAudioEndpointVolume = self
    //         .default_device
    //         .clone()
    //         .unwrap()
    //         .Activate(CLSCTX_ALL, None)
    //         .unwrap_or_else(|err| {
    //             eprintln!("ERROR: Couldn't get Endpoint volume control: {err}");
    //             exit(1);
    //         });
    //
    //     self.sessions.push(Box::new(EndPointSession::new(
    //         simple_audio_volume,
    //         "master".to_string(),
    //         0,
    //     )));
    // }

    unsafe fn get_sessions() -> Vec<Box<dyn Session>> {
        let mut sessions: Vec<Box<dyn Session>> = Vec::new();

        for device in Self::get_devices() {
            let session_manager2: IAudioSessionManager2 = device.Activate(CLSCTX_INPROC_SERVER, None).unwrap_or_else(|err| {
                eprintln!("ERROR: Couldnt get AudioSessionManager for enumerating over processes... {err}");
                exit(1);
            });

            let session_enumerator = session_manager2.GetSessionEnumerator();

            let session_enumerator = match session_enumerator {
                Ok(x) => x,
                Err(err) => {
                    eprintln!("ERROR: Couldn't get session enumerator: {err}");
                    continue;
                }
            };

            for i in 0..session_enumerator.GetCount().unwrap() {
                let normal_session_control: Option<IAudioSessionControl> =
                    session_enumerator.GetSession(i).ok();
                if normal_session_control.is_none() {
                    eprintln!("ERROR: Couldn't get session control of audio session...");
                    continue;
                }

                let session_control: Option<IAudioSessionControl2> =
                    normal_session_control.unwrap().cast().ok();
                if session_control.is_none() {
                    eprintln!(
                        "ERROR: Couldn't convert from normal session control to session control 2"
                    );
                    continue;
                }

                let pid = session_control.as_ref().unwrap().GetProcessId().unwrap();
                if pid == 0 {
                    continue;
                }
                let process =
                    OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid).ok();
                let process = match process {
                    Some(data) => data,
                    None => {
                        eprintln!("ERROR: Couldn't get process information of process id {pid}");
                        continue;
                    }
                };
                let mut filename: [u8; 128] = [0; 128];
                K32GetProcessImageFileNameA(process, &mut filename);
                let mut new_filename: Vec<u8> = vec![];
                for i in filename.iter() {
                    if i == &(0 as u8) {
                        continue;
                    }
                    new_filename.push(i.clone());
                }
                let mut str_filename = match String::from_utf8(new_filename) {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("ERROR: Filename couldn't be converted to string, {err}");
                        continue;
                    }
                };
                str_filename = match str_filename.split("\\").last() {
                    Some(data) => data.to_string().replace(".exe", ""),
                    None => {
                        continue;
                    }
                };
                let audio_control: ISimpleAudioVolume = match session_control.unwrap().cast() {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!(
                        "ERROR: Couldn't get the simpleaudiovolume from session controller: {err}"
                    );
                        continue;
                    }
                };
                let application_session = ApplicationSession::new(audio_control, str_filename, pid);
                sessions.push(Box::new(application_session));
            }
        }

        let mut pids = HashSet::new();
        sessions.retain(|s| {
            let is_first = !pids.contains(&s.get_pid());
            pids.insert(s.get_pid());
            is_first && !s.get_name().is_empty()
        });

        sessions
    }

    pub unsafe fn get_all_session_names(&self) -> Vec<String> {
        self.sessions.iter().map(|i| i.get_name()).collect()
    }

    pub unsafe fn get_session_by_name(&self, name: String) -> Option<&Box<dyn Session>> {
        self.sessions.iter().find(|i| i.get_name() == name)
    }
}
