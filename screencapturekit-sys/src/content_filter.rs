use objc::{runtime::Class, *};
use objc_foundation::{INSArray, INSObject, NSArray};
use objc_id::{Id, ShareId, Shared};
use runtime::Object;

use super::shareable_content::{UnsafeSCDisplay, UnsafeSCRunningApplication, UnsafeSCWindow};

#[derive(Debug)]
pub struct UnsafeContentFilter {
    __priv: u8,
}
unsafe impl Message for UnsafeContentFilter {}

impl INSObject for UnsafeContentFilter {
    fn class() -> &'static Class {
        Class::get("SCContentFilter").expect(
            "Missing SCContentFilter class, check that the binary is linked with ScreenCaptureKit",
        )
    }
}

pub enum UnsafeInitParams {
    DesktopIndependentWindow(ShareId<UnsafeSCWindow>),
    Display(ShareId<UnsafeSCDisplay>),
    DisplayIncludingWindows(ShareId<UnsafeSCDisplay>, Vec<ShareId<UnsafeSCWindow>>),
    DisplayExcludingWindows(ShareId<UnsafeSCDisplay>, Vec<ShareId<UnsafeSCWindow>>),
    DisplayIncludingApplicationsExceptingWindows(
        ShareId<UnsafeSCDisplay>,
        Vec<ShareId<UnsafeSCRunningApplication>>,
        Vec<ShareId<UnsafeSCWindow>>,
    ),
    DisplayExcludingApplicationsExceptingWindows(
        ShareId<UnsafeSCDisplay>,
        Vec<ShareId<UnsafeSCRunningApplication>>,
        Vec<ShareId<UnsafeSCWindow>>,
    ),
}

impl UnsafeContentFilter {
    pub fn init(params: UnsafeInitParams) -> Id<Self> {
        unsafe {
            let alloc: *mut Object = msg_send![Self::class(), alloc];
            let init_result: *mut Object = match params {
                UnsafeInitParams::Display(display) => objc::rc::autoreleasepool(
                    || msg_send![alloc, initWithDisplay:display excludingWindows:NSArray::from_slice(&[] as &[Id<UnsafeSCWindow, Shared>])],
                ),
                UnsafeInitParams::DesktopIndependentWindow(window) => objc::rc::autoreleasepool(
                    || msg_send![alloc, initWithDesktopIndependentWindow:window],
                ),
                UnsafeInitParams::DisplayIncludingWindows(display, windows) => {
                    objc::rc::autoreleasepool(
                        || msg_send![alloc, initWithDisplay:display includingWindows:NSArray::from_vec(windows)],
                    )
                }
                UnsafeInitParams::DisplayExcludingWindows(display, windows) => {
                    objc::rc::autoreleasepool(
                        || msg_send![alloc, initWithDisplay:display excludingWindows:NSArray::from_vec(windows)],
                    )
                }
                UnsafeInitParams::DisplayIncludingApplicationsExceptingWindows(
                    display,
                    applications,
                    windows,
                ) => objc::rc::autoreleasepool(
                    || msg_send![alloc, initWithDisplay:display includingApplications:NSArray::from_vec(applications) exceptingWindows:NSArray::from_vec(windows)],
                ),
                UnsafeInitParams::DisplayExcludingApplicationsExceptingWindows(
                    display,
                    applications,
                    windows,
                ) => objc::rc::autoreleasepool(
                    || msg_send![alloc, initWithDisplay:display excludingApplications:NSArray::from_vec(applications) exceptingWindows:NSArray::from_vec(windows)],
                ),
            };

            if init_result.is_null() {
                panic!("Failed to initialize UnsafeContentFilter");
            }

            Id::from_ptr(init_result as *mut Self)
        }
    }
}

#[cfg(test)]
mod test_content_filter {
    use super::*;
    use crate::shareable_content::UnsafeSCShareableContent;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_init() {
        let sc = UnsafeSCShareableContent::get().expect("should get shareable content");
        let applications = sc.applications();
        let windows = sc.windows();
        let display = sc.displays().pop().unwrap();

        UnsafeContentFilter::init(UnsafeInitParams::DisplayIncludingWindows(
            display.clone(),
            windows.clone(),
        ));
        UnsafeContentFilter::init(UnsafeInitParams::DisplayExcludingWindows(
            display.clone(),
            windows.clone(),
        ));
        UnsafeContentFilter::init(UnsafeInitParams::DesktopIndependentWindow(
            windows[0].clone(),
        ));
        UnsafeContentFilter::init(
            UnsafeInitParams::DisplayIncludingApplicationsExceptingWindows(
                display.clone(),
                applications.clone(),
                windows.clone(),
            ),
        );
        UnsafeContentFilter::init(
            UnsafeInitParams::DisplayExcludingApplicationsExceptingWindows(
                display.clone(),
                applications.clone(),
                windows.clone(),
            ),
        );

        drop(sc);
        drop(applications);
        drop(windows);
        drop(display);
    }
}
