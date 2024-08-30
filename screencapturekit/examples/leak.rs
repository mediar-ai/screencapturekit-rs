use std::process::Command;

use screencapturekit::{
    cm_sample_buffer::CMSampleBuffer,
    sc_content_filter::{InitParams, SCContentFilter},
    sc_error_handler::StreamErrorHandler,
    sc_output_handler::{SCStreamOutputType, StreamOutput},
    sc_shareable_content::SCShareableContent,
    sc_stream::SCStream,
    sc_stream_configuration::{PixelFormat, SCStreamConfiguration},
    sc_types::base::CMTime,
};
use screencapturekit_sys::{
    content_filter::{UnsafeContentFilter, UnsafeInitParams},
    shareable_content::UnsafeSCShareableContent,
};

pub struct Capturer {}

impl Capturer {
    pub fn new() -> Self {
        println!("Capturer initialized");
        Capturer {}
    }
}

impl StreamErrorHandler for Capturer {
    fn on_error(&self) {
        eprintln!("ERROR!");
    }
}

impl StreamOutput for Capturer {
    fn did_output_sample_buffer(&self, _sample: CMSampleBuffer, _of_type: SCStreamOutputType) {
        println!("New frame recvd");
    }
}
fn main() {
    println!("Starting");

    for _ in 0..1 {
        // Repeat the process multiple times to amplify leaks
        // Create SCShareableContent and SCContentFilter
        let display = SCShareableContent::current().displays.pop().unwrap();
        let windows = SCShareableContent::current().windows;

        // Create multiple filters
        let _filter1 = SCContentFilter::new(InitParams::DisplayExcludingWindows(
            display.clone(),
            windows,
        ));
        let _filter2 = SCContentFilter::new(InitParams::Display(display.clone()));
        let _filter3 =
            SCContentFilter::new(InitParams::DisplayExcludingWindows(display.clone(), vec![]));

        // Create multiple configurations
        let _config1 = SCStreamConfiguration {
            width: 1920,
            height: 1080,
            ..Default::default()
        };
        let _config2 = SCStreamConfiguration {
            width: 1280,
            height: 720,
            ..Default::default()
        };

        // Create and immediately drop streams
        let init_params = InitParams::Display(display);
        let filter = SCContentFilter::new(init_params);
        let mut sc_stream = SCStream::new(filter, _config1, Capturer {});
        let output = Capturer {};
        sc_stream.add_output(output, SCStreamOutputType::Screen);

        // Force drop of sc_stream
        drop(sc_stream);
    }

    // Get the current process ID
    let pid = std::process::id();

    // Run the 'leaks' command
    let output = Command::new("leaks")
        .args(&[pid.to_string()])
        .output()
        .expect("Failed to execute leaks command");

    // Check the output for leaks
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("leaks stdout: {}", stdout);
    println!("leaks stderr: {}", stderr);
}
