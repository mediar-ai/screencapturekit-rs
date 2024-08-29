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
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
        println!("New frame recvd");
    }
}
fn main() {
    println!("Starting");
    // Create SCShareableContent and SCContentFilter
    let display = SCShareableContent::current().displays.pop().unwrap();
    let windows = SCShareableContent::current().windows;
    let _filter = SCContentFilter::new(InitParams::DisplayExcludingWindows(display, windows));
    let display = SCShareableContent::current().displays.pop().unwrap();
    let display = SCShareableContent::current().displays.pop().unwrap();

    let _filter = SCContentFilter::new(InitParams::Display(display));
    let display = SCShareableContent::current().displays.pop().unwrap();

    let _filter = SCContentFilter::new(InitParams::DisplayExcludingWindows(display, vec![]));
    
    let config = SCStreamConfiguration {
        width: 1920,
        height: 1080,
        ..Default::default()
    };
    let display = SCShareableContent::current().displays.pop().unwrap();

    let init_params = InitParams::Display(display);
    let filter = SCContentFilter::new(init_params);
    let mut sc_stream = SCStream::new(filter, config, Capturer {});
    let output = Capturer {};
    sc_stream.add_output(output, SCStreamOutputType::Audio);
    let playing = false;
    // let stream = Stream::new(StreamInner { sc_stream, playing });
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
