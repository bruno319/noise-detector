use std::sync::mpsc::*;

use portaudio;

use crate::input::{get_input_settings, Sensibility, NoiseState};
use crate::output::Alarm;
use std::io::stdin;

mod output;

mod input;

fn main() {
    let sensibility = choose_sensibility_level();
    let mut noise_state = NoiseState::new(sensibility);

    let pa = portaudio::PortAudio::new().expect("Unable to init PortAudio");
    let mut alarm = Alarm::new(&pa);
    let input_settings = get_input_settings(&pa);

    let (sender, receiver) = channel();

    let read_input_callback = move |portaudio::InputStreamCallbackArgs { buffer, .. }| {
        match sender.send(buffer) {
            Ok(_) => portaudio::Continue,
            Err(_) => portaudio::Complete
        }
    };

    let mut stream = pa.open_non_blocking_stream(input_settings, read_input_callback).expect("Unable to create stream");
    stream.start().expect("Unable to start stream");

    while stream.is_active().unwrap() {
        if noise_state.is_noisy() && !alarm.is_playing() {
            alarm.play(&pa).expect("Unable to play alarm sound");
        }

        while let Ok(buffer) = receiver.try_recv() {
            noise_state.add(buffer.to_vec());
        }
    }
}

fn choose_sensibility_level() -> Sensibility {
    print_menu();
    let option = &mut String::new();
    stdin().read_line(option)
        .map_or_else(|_err| {
            println!("Unable to read console. Setting to medium sensibility");
            Sensibility::Medium
        }, |_ok| set_sensibility_option(option))
}

fn print_menu() {
    println!("Choose sensibility level: \n \
    1 - High \n \
    2 - Medium \n \
    3 - Low");
}

fn set_sensibility_option(option: &mut String) -> Sensibility {
    match option.trim() {
        "1" => Sensibility::High,
        "2" => Sensibility::Medium,
        "3" => Sensibility::Low,
        _ => {
            println!("Invalid option. Setting to medium sensibility");
            Sensibility::Medium
        }
    }
}