#![windows_subsystem = "windows"]
use anyhow::Result;
use serde::Deserialize;
use tokio_stream::StreamExt;
use winapi::um::winuser::{
    INPUT_u, MapVirtualKeyA, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MAPVK_VK_TO_VSC,
};
use wmi::{COMLibrary, WMIConnection};

use std::mem::size_of;

const VK_HOME: u32 = 0x24;
const OMEN_EVENT_ID: u32 = 29;
const OMEN_EVENT_DATA: u32 = 8613;
const NAMESPACE_PATH: &str = "root\\WMI";

#[derive(Deserialize, Debug)]
#[serde(rename = "hpqBEvnt")]
struct OmenEvent {
    #[serde(rename = "eventId")]
    event_id: u32,
    #[serde(rename = "EventData")]
    event_data: u32,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let w_scan = unsafe { MapVirtualKeyA(VK_HOME, MAPVK_VK_TO_VSC) };
    let input = unsafe {
        let mut u: INPUT_u = std::mem::zeroed();
        *u.ki_mut() = KEYBDINPUT {
            wVk: VK_HOME as u16,
            wScan: w_scan as u16,
            dwFlags: KEYEVENTF_EXTENDEDKEY | KEYEVENTF_SCANCODE,
            time: 0,
            dwExtraInfo: 0,
        };
        let press = INPUT {
            type_: INPUT_KEYBOARD,
            u,
        };
        let unpress = {
            let mut copy = press;
            copy.u.ki_mut().dwFlags |= KEYEVENTF_KEYUP;
            copy
        };
        [press, unpress]
    };
    let wmi_con = WMIConnection::with_namespace_path(NAMESPACE_PATH, COMLibrary::new()?)?;
    let mut notifications = wmi_con.async_notification::<OmenEvent>()?;
    while let Some(notification) = notifications.next().await {
        match notification {
            Ok(OmenEvent {
                event_id: OMEN_EVENT_ID,
                event_data: OMEN_EVENT_DATA,
            }) => unsafe {
                let mut input = input;
                SendInput(
                    input.len() as u32,
                    &mut input as *mut INPUT,
                    size_of::<INPUT>() as i32,
                );
            },
            Err(e) => {
                dbg!(e);
            }
            _ => {}
        }
    }
    Ok(())
}
