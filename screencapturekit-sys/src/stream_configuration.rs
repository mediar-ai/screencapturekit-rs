use objc::{msg_send, runtime::Class, *};

use objc_foundation::INSObject;
use objc_id::Id;
use runtime::Object;

use crate::os_types::{
    base::{CMTime, OSType, UInt32, BOOL},
    four_char_code::FourCharCode,
    geometry::CGRect,
    graphics::CGColor,
};
// Implement Encode for CGRect
unsafe impl Encode for CGRect {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") }
    }
}

// Implement Encode for OSType (assuming it's a typedef for u32)
unsafe impl Encode for OSType {
    fn encode() -> Encoding {
        u32::encode()
    }
}

// Implement Encode for CMTime
unsafe impl Encode for CMTime {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CMTime=qiIq}") }
    }
}

#[derive(Debug)]
pub struct UnsafeStreamConfigurationRef;
unsafe impl Message for UnsafeStreamConfigurationRef {}
impl From<UnsafeStreamConfiguration> for Id<UnsafeStreamConfigurationRef> {
    fn from(value: UnsafeStreamConfiguration) -> Self {
        let unsafe_config: UnsafeStreamConfiguration = value;
        objc::rc::autoreleasepool(|| unsafe {
            let cls = UnsafeStreamConfigurationRef::class();
            let obj: *mut UnsafeStreamConfigurationRef = msg_send![cls, alloc];
            let obj: *mut UnsafeStreamConfigurationRef = msg_send![obj, init];
            let obj = Id::from_ptr(obj);

            let _: () = msg_send![&*obj, setWidth:unsafe_config.width];
            let _: () = msg_send![&*obj, setHeight:unsafe_config.height];
            let _: () = msg_send![&*obj, setCapturesAudio:unsafe_config.captures_audio];
            let _: () = msg_send![&*obj, setSourceRect:unsafe_config.source_rect];
            let _: () = msg_send![&*obj, setDestinationRect:unsafe_config.destination_rect];
            let _: () = msg_send![&*obj, setPixelFormat:unsafe_config.pixel_format];
            let _: () =
                msg_send![&*obj, setMinimumFrameInterval:unsafe_config.minimum_frame_interval];
            let _: () = msg_send![&*obj, setScalesToFit:unsafe_config.scales_to_fit];
            let _: () = msg_send![&*obj, setShowsCursor:unsafe_config.shows_cursor];
            let _: () = msg_send![&*obj, setChannelCount:unsafe_config.channel_count];
            let _: () = msg_send![&*obj, setSampleRate:unsafe_config.sample_rate];
            let _: () =
                msg_send![&*obj, setPreservesAspectRatio:unsafe_config.preserves_aspect_ratio];

            obj
        })
    }
}
impl INSObject for UnsafeStreamConfigurationRef {
    fn class() -> &'static Class {
        Class::get("SCStreamConfiguration")
                .expect("Missing SCStreamConfiguration class, check that the binary is linked with ScreenCaptureKit")
    }
}

impl Drop for UnsafeStreamConfigurationRef {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self, release];
        }
    }
}

#[derive(Debug)]
pub struct UnsafeStreamConfiguration {
    // The width of the output.
    pub width: UInt32,
    //   The height of the output.
    pub height: UInt32,
    // A boolean value that indicates whether to scale the output to fit the configured width and height.
    pub scales_to_fit: BOOL,
    // A rectangle that specifies the source area to capture.
    pub source_rect: CGRect,
    // A rectangle that specifies a destination into which to write the output.
    pub destination_rect: CGRect,
    // A Boolean value that determines if the stream preserves aspect ratio.
    pub preserves_aspect_ratio: BOOL,
    // Configuring Colors

    // A pixel format for sample buffers that a stream outputs.
    pub pixel_format: OSType,
    // A color matrix to apply to the output surface.
    pub color_matrix: String,
    // A color space to use for the output buffer.
    pub color_space_name: String,
    // A background color for the output.
    // Controlling Visibility
    pub background_color: CGColor,

    // A boolean value that determines whether the cursor is visible in the stream.
    pub shows_cursor: BOOL,
    // Optimizing Performance
    // The maximum number of frames for the queue to store.
    pub queue_depth: UInt32,
    // The desired minimum time between frame updates, in seconds.
    pub minimum_frame_interval: CMTime,
    // Configuring Audio
    // A boolean value that indicates whether to capture audio.
    pub captures_audio: BOOL,
    // The sample rate for audio capture.
    pub sample_rate: UInt32,
    // The number of audio channels to capture.
    pub channel_count: UInt32,
    // A boolean value that indicates whether to exclude a
    pub excludes_current_process_audio: BOOL,
}

impl Default for UnsafeStreamConfiguration {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            scales_to_fit: 0,
            preserves_aspect_ratio: 1,
            source_rect: Default::default(),
            destination_rect: Default::default(),
            pixel_format: FourCharCode::from_chars(*b"BGRA"),
            color_matrix: Default::default(),
            color_space_name: Default::default(),
            background_color: Default::default(),
            shows_cursor: Default::default(),
            queue_depth: Default::default(),
            minimum_frame_interval: Default::default(),
            captures_audio: Default::default(),
            sample_rate: Default::default(),
            channel_count: Default::default(),
            excludes_current_process_audio: Default::default(),
        }
    }
}

#[cfg(test)]
mod get_shareable_content {

    use super::*;
    #[test]
    fn test_from() {
        let _: Id<UnsafeStreamConfigurationRef> = UnsafeStreamConfiguration::default().into();
    }
}
