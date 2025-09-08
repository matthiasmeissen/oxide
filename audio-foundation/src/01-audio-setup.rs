use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


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

    // This is a closure we pass to cpal in .build_output_stream
    // The move keyword forces it to take ownership over any variables
    // data is a mutable slice representing the hardware buffer that needs to be filled
    // Inside the callback we iterate over every item in the data slice and set its value to zero
    // Which creates a stream of slience in that case
    let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            *sample = 0.0;
        }
    };

    // This is a closure we can use to shor error messages
    let err_fn = |err| eprintln!("An error occured on the audio stream: {}", err);

    // When our sample format is f32 we call the .build_output_stream method 
    // And store it the a stream variable
    // The method takes in different parameters 
    // config: reference to the stream_config struct (channels, sample_rate, buffer_size) 
    // data_callback: the audio_callback closure that is called (how to fill the buffer)
    // err_callback: the err_fn closure that us called when something goes wrong
    // timeout: an optional value that sets the limit it should take to build the stream
    // The .expect() method is called when it is not possible to build the stream
    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &stream_config,
            audio_callback,
            err_fn,
            None
        ),
        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
    }.expect("Could not build f32 output stream.");

    // The .play() method starts the audio processing
    // It signals the audio thread to start calling the audio_callback
    stream.play().expect("Could not start audio stream.");
    println!("Audio pipeline is running.");

    // The stream keeps owning the audio hardware as long it is active
    // This means it only drops when it gets out of scope, which would be when main ends
    // By using sleep we put the current thread to sleep for the specified amount of time
    // After that the program is done
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Program finished.");

}
