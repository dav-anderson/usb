use std::time::Duration;
use std::thread;
mod usb; // Declare the usb module
use usb::UsbInfo; // Import UsbInfo, Platform, and Error from usb.rs
fn main() -> Result<(), String> {
    // Specify the platform (Linux in this case)
    let platform = "macos";
    // Initialize UsbInfo, handle potential errors using ?
    let mut usb_info: UsbInfo = UsbInfo::new(&platform.to_string())?;
    // Main loop to detect new USB devices
    loop {
        // Detect new device path
        if let Some(device_path) = usb_info.detect_new_device_path(&platform.to_string())? {
            // Convert Option<PathBuf> to String safely
            let device_path_str = device_path.to_string_lossy().into_owned();
            println!("New device path: {}", device_path_str);
        }
        else{
            println!("None");
        }
        // Sleep for 1 second before the next iteration to avoid busy looping
        thread::sleep(Duration::from_millis(1_000));
    }
}