use std::collections::VecDeque;

use portaudio::PortAudio;
use portaudio::stream::InputSettings;

#[derive(Debug)]
pub enum Sensibility {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct NoiseState {
    sensibility: Sensibility,
    sound_values: VecDeque<f32>,
}

impl NoiseState {
    pub fn new(mic_volume: Sensibility) -> NoiseState {
        NoiseState {
            sensibility: mic_volume,
            sound_values: VecDeque::with_capacity(7500),
        }
    }

    pub fn add(&mut self, sound_values: Vec<f32>) {
        if self.sound_values.len() == 7500 {
            self.sound_values.pop_front();
        }
        let mut sound_values: VecDeque<f32> = VecDeque::from(sound_values);
        self.sound_values.append(&mut sound_values);
    }

    pub fn is_noisy(&self) -> bool {
        let sum: f32 = self.sound_values
            .iter()
            .map(|value| value + value.abs())
            .sum();
        let average = sum / self.sound_values.len() as f32;
        average > self.sensibility.get_sensibility_value()
    }
}

impl Sensibility {
    fn get_sensibility_value(&self) -> f32 {
        match self {
            Self::Low => 1.3,
            Self::Medium => 0.8,
            Self::High => 0.3
        }
    }
}

pub fn get_input_settings(portaudio: &PortAudio) -> InputSettings<f32> {
    let mic_index = portaudio.default_input_device().expect("Unable to get default device");
    let mic = portaudio.device_info(mic_index).expect("unable to get mic info");

    let input_params = portaudio::StreamParameters::<f32>::new(mic_index, 1, true, mic.default_low_input_latency);
    portaudio::InputStreamSettings::new(input_params, mic.default_sample_rate, 1)
}