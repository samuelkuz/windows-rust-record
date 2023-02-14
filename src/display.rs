use windows::Graphics::Capture::GraphicsCaptureItem;
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW,
};
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;

use crate::result::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct Display {
    pub handle: HMONITOR,
    pub display_name: String,
    pub resolution: (u32, u32),
}

impl Display {
    fn new(monitor_handle: HMONITOR) -> Result<Self> {
        let mut info = MONITORINFOEXW::default();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

        unsafe {
            GetMonitorInfoW(monitor_handle, &mut info as *mut _ as *mut _).ok()?;
        }

        let display_name = String::from_utf16_lossy(&info.szDevice)
            .trim_matches(char::from(0))
            .to_string();

        Ok(Self {
            handle: monitor_handle,
            display_name,
            resolution: ((info.monitorInfo.rcMonitor.right - info.monitorInfo.rcMonitor.left) as u32,
                         (info.monitorInfo.rcMonitor.bottom - info.monitorInfo.rcMonitor.top) as u32),
        })
    }

    pub fn enumerate_displays() -> Result<Box<Vec<Display>>> {
        unsafe {
            let displays = Box::into_raw(Box::new(Vec::<Display>::new()));
            EnumDisplayMonitors(
                HDC(0),
                None,
                Some(enum_monitor),
                LPARAM(displays as isize),
            );
            Ok(Box::from_raw(displays))
        }
    }

    pub fn primary_display() -> Option<Display> {
        let mut displays = Display::enumerate_displays().unwrap();
        
        if displays.is_empty() {
            return None;
        }

        Some(displays.swap_remove(0))
    }
}

pub fn create_capture_item_for_monitor(monitor_handle: HMONITOR) -> Result<GraphicsCaptureItem> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    Ok(unsafe { interop.CreateForMonitor(monitor_handle) }?)
}


extern "system" fn enum_monitor(monitor: HMONITOR, _: HDC, _: *mut RECT, state: LPARAM) -> BOOL {
    unsafe {
        let state = Box::leak(Box::from_raw(state.0 as *mut Vec<Display>));
        state.push(Display::new(monitor).unwrap());
    }
    true.into()
}
