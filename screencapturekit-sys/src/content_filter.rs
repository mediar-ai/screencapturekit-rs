use objc::{runtime::Class, *};
use objc_foundation::{INSArray, INSObject, NSArray};
use objc_id::{Id, Owned, ShareId, Shared};
use runtime::Object;

use crate::as_ptr::AsPtr;

use super::shareable_content::{UnsafeSCDisplay, UnsafeSCRunningApplication, UnsafeSCWindow};
use objc::Message;

pub struct SafeNSArray<T: Message + INSObject>(Id<NSArray<T>>);
impl<T: Message + INSObject> SafeNSArray<T> {
    pub fn new(vec: Vec<Id<T, Shared>>) -> Self {
        let refs: Vec<&T> = vec.iter().map(|obj| &**obj).collect();
        unsafe {
            let array: *mut NSArray<T> = msg_send![NSArray::<T>::class(), alloc];
            let array: *mut NSArray<T> = msg_send![array,
                initWithObjects:refs.as_ptr()
                count:refs.len()
            ];
            SafeNSArray(Id::from_ptr(array))
        }
    }

    pub fn as_inner(&self) -> &NSArray<T> {
        &self.0
    }
}

impl<T: Message + INSObject> std::ops::Deref for SafeNSArray<T> {
    type Target = NSArray<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Message + INSObject> Drop for SafeNSArray<T> {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![&*self.0, release];
        }
    }
}

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
    DesktopIndependentWindow(Id<UnsafeSCWindow, Shared>),
    Display(Id<UnsafeSCDisplay, Shared>),
    DisplayIncludingWindows(Id<UnsafeSCDisplay, Shared>, Vec<Id<UnsafeSCWindow, Shared>>),
    DisplayExcludingWindows(Id<UnsafeSCDisplay, Shared>, Vec<Id<UnsafeSCWindow, Shared>>),
    DisplayIncludingApplicationsExceptingWindows(
        Id<UnsafeSCDisplay, Shared>,
        Vec<Id<UnsafeSCRunningApplication, Shared>>,
        Vec<Id<UnsafeSCWindow, Shared>>,
    ),
    DisplayExcludingApplicationsExceptingWindows(
        Id<UnsafeSCDisplay, Shared>,
        Vec<Id<UnsafeSCRunningApplication, Shared>>,
        Vec<Id<UnsafeSCWindow, Shared>>,
    ),
}

impl UnsafeContentFilter {
    pub fn init(params: UnsafeInitParams) -> Id<Self> {
        unsafe {
            objc::rc::autoreleasepool(|| {
                let alloc: *mut Object = msg_send![Self::class(), alloc];
                let init_result: *mut Object = match params {
                    UnsafeInitParams::Display(display) => {
                        let empty_array = SafeNSArray::<UnsafeSCWindow>::new(Vec::new());
                        msg_send![alloc, initWithDisplay:display excludingWindows:empty_array.as_inner()]
                    }
                    UnsafeInitParams::DesktopIndependentWindow(window) => {
                        msg_send![alloc, initWithDesktopIndependentWindow:window]
                    }
                    UnsafeInitParams::DisplayIncludingWindows(display, windows) => {
                        let ns_array = SafeNSArray::new(windows);
                        msg_send![alloc, initWithDisplay:display includingWindows:ns_array.as_inner()]
                    }
                    UnsafeInitParams::DisplayExcludingWindows(display, windows) => {
                        let ns_array = SafeNSArray::new(windows);
                        msg_send![alloc, initWithDisplay:display excludingWindows:ns_array.as_inner()]
                    }
                    UnsafeInitParams::DisplayIncludingApplicationsExceptingWindows(
                        display,
                        applications,
                        windows,
                    ) => {
                        let apps_array = SafeNSArray::new(applications);
                        let windows_array = SafeNSArray::new(windows);
                        msg_send![alloc, initWithDisplay:display includingApplications:apps_array.as_inner() exceptingWindows:windows_array.as_inner()]
                    }
                    UnsafeInitParams::DisplayExcludingApplicationsExceptingWindows(
                        display,
                        applications,
                        windows,
                    ) => {
                        let apps_array = SafeNSArray::new(applications);
                        let windows_array = SafeNSArray::new(windows);
                        msg_send![alloc, initWithDisplay:display excludingApplications:apps_array.as_inner() exceptingWindows:windows_array.as_inner()]
                    }
                };

                if init_result.is_null() {
                    panic!("Failed to initialize UnsafeContentFilter");
                }

                Id::from_ptr(init_result as *mut Self)
            })
        }
    }
}

impl Drop for UnsafeContentFilter {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self, release];
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
