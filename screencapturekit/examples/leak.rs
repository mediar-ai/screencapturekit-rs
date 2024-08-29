use std::process::Command;

use screencapturekit::{
    sc_content_filter::{InitParams, SCContentFilter},
    sc_shareable_content::SCShareableContent,
};
use screencapturekit_sys::{
    content_filter::{UnsafeContentFilter, UnsafeInitParams},
    shareable_content::UnsafeSCShareableContent,
};

fn main() {
    println!("Starting");
    // Create SCShareableContent and SCContentFilter
    let display = SCShareableContent::current().displays.pop().unwrap();
    let windows = SCShareableContent::current().windows;
    let _filter = SCContentFilter::new(InitParams::DisplayExcludingWindows(display, windows));
    // let display = SCShareableContent::current().displays.pop().unwrap();
    // let _filter = SCContentFilter::new(InitParams::Display(display));
    // let _filter = SCContentFilter::new(InitParams::DisplayExcludingWindows(display, vec![]));
    // unsafe {
    //     let unsafe_display = UnsafeSCShareableContent::displays()
    //         .unwrap()
    //         .first()
    //         .unwrap();
    //     let _filter = UnsafeContentFilter::init(UnsafeInitParams::Display(display));
    // }
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
