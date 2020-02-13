# noise-detector

A simple project using Rust that captures the audio from default microphone and sounds an alarm when the noise is loud.

## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)

- [Portaudio](http://www.portaudio.com)

To install portaudio on Ubuntu:
```
sudo apt-get install libasound-dev portaudio19-dev
```

## how to run
Clone the project
```
git clone https://github.com/bruno319/noise-detector
```
Enter in folder project
```
cd noise-detector
```
Change rust channel to nightly
```
rustup default nightly
```
Run the project
```
cargo run
```
