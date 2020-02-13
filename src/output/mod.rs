use portaudio;
use portaudio::{Stream, NonBlocking, Output, PortAudio};
use portaudio::stream::OutputSettings;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

pub struct Alarm {
    playing: bool,
    stream: Stream<NonBlocking, Output<f32>>
}

impl Alarm {
    pub fn new(portaudio: &portaudio::PortAudio) -> Alarm {
        let mut left_saw = 0.0;
        let mut right_saw = 0.0;

        let sound_wave_callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, .. }| {
            let mut idx = 0;
            for _ in 0..frames {
                buffer[idx] = left_saw;
                buffer[idx + 1] = right_saw;
                left_saw += 0.01;
                if left_saw >= 1.0 {
                    left_saw -= 2.0;
                }
                right_saw += 0.03;
                if right_saw >= 1.0 {
                    right_saw -= 2.0;
                }
                idx += 2;
            }
            portaudio::Continue
        };
        let stream = portaudio
            .open_non_blocking_stream(get_default_audio_output_settings(portaudio), sound_wave_callback)
            .expect("Unable to open output stream");

        Alarm {
            playing: false,
            stream
        }
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    pub fn play(&mut self, portaudio: &portaudio::PortAudio) -> Result<(), portaudio::Error> {
        self.playing = true;

        self.stream.start()?;
        portaudio.sleep(1_000);
        self.stream.stop()?;

        self.playing = false;
        Ok(())
    }
}

fn get_default_audio_output_settings(portaudio: &PortAudio) -> OutputSettings<f32> {
    let mut settings = portaudio.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
        .expect("Unable to locate default audio output");
    settings.flags = portaudio::stream_flags::CLIP_OFF;
    settings
}
