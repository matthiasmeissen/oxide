use cpal::traits::{DeviceTrait, HostTrait};


fn main() {
    println!("Setting up audio pipeline.");

    // Get the default audio host (provides access to available audio devices)
    let host = cpal::default_host();
    println!("Audio Host: {}", host.id().name());

    // Find the default output device
    let device = host.default_output_device().expect("No output device available.");
    println!("Output device: {:?}", device.name());

    // Get default stream configuration (Includes preferred sample rate and channel count)
    let config = device.default_output_config().expect("No default output config found.");
    println!("Default output config: {:#?}", config);

    // Store sample format from config as variable to use it later
    let sample_format = config.sample_format();
    println!("Expected sample format: {}", sample_format);

    // Get stream configuration struct
    let stream_config: cpal::StreamConfig = config.into();
    println!("Stream config: {:#?}", stream_config);
}
