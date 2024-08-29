use std::process::Command;

use screencapturekit::{
    sc_content_filter::SCContentFilter, sc_shareable_content::SCShareableContent,
};

fn main() {
    println!("Starting");
    // Create SCShareableContent and SCContentFilter
    let _display = SCShareableContent::current().displays.pop().unwrap();
    // let _filter = SCContentFilter::new(InitParams::Display(display));

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
