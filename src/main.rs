use std::time::Duration;
use std::thread;
mod usb; // Declare the usb module
use usb::UsbInfo; // Import UsbInfo, Platform, and Error from usb.rs
fn main() -> Result<(), String> {
    // Specify the platform (Linux in this case)
    let platform = "linux";
    // Initialize UsbInfo, handle potential errors using ?
    let mut usb_info: UsbInfo = UsbInfo::new(&platform.to_string())?;
    // Main loop to detect new USB devices
    loop {
        // Detect new device path
        if let Some(device_path) = usb_info.detect_new_device_path(&platform.to_string())? {
            // Convert Option<PathBuf> to String safely
            let device_path_str = device_path.to_string_lossy().into_owned();
            println!("New device path: {}", device_path_str);
            // Match on the result of write_to_file to handle errors
            match UsbInfo::write_to_file(&device_path_str, "hello world", &platform.to_string()) { 
                Ok(_) => println!("File written successfully."), 
                Err(e) => eprintln!("Error writing to file: {}", e), 
            }            
            let read = UsbInfo::read_from_file(&device_path_str, &platform.to_string()).unwrap();
            println!("reading file contents: {}", &read);
        }
        else{
            println!("None");
        }
        // Sleep for 1 second before the next iteration to avoid busy looping
        thread::sleep(Duration::from_millis(1_000));
    }
}