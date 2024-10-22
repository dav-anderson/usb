use std::collections::HashSet;
use std::env;
use std::path::{PathBuf};
use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};


pub struct UsbInfo {
    pub baseline: HashSet<PathBuf>, // Store baseline as a HashSet<PathBuf> objects
}impl UsbInfo {
    // Constructor for UsbInfo that initializes the baseline as a HashSet<PathBuf>
    pub fn new(platform: &str) -> Result<Self, String> {
        Ok(UsbInfo {
            baseline: UsbInfo::query_devices(&platform)?.into_iter().collect(),
        })
    }    // Parse the command output and convert to Vec<PathBuf>
    fn parse_command_output(output: Vec<u8>) -> Vec<PathBuf> {
        let result = String::from_utf8_lossy(&output);
        result
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| PathBuf::from(line.trim()))
            .collect()
    }    // Run ls command on a given path
    fn run_ls_command(path: &PathBuf) -> Result<Vec<PathBuf>, String> {  // Change to PathBuf instead of String
        let output = Command::new("ls")
            .arg(path.as_os_str())  // Use .as_os_str() to convert PathBuf to OsStr
            .output();
        Ok(UsbInfo::parse_command_output(output.unwrap().stdout))
    }    // Method to query devices and return a vector of PathBufs
    pub fn query_devices(platform: &str) -> Result<Vec<PathBuf>, String> {
        let mut device_paths: Vec<PathBuf> = Vec::new();
        match platform {
            "windows" => {
                // On Windows, use `wmic` to list logical disks (drives)
                let output = Command::new("wmic")
                    .args(&["logicaldisk", "get", "name"])
                    .output();
                device_paths = UsbInfo::parse_command_output(output.unwrap().stdout);
            }
            "linux" => {
                if let Some(user) = env::var_os("USER") {
                    let media_path = PathBuf::from(format!("/media/{}", user.to_string_lossy()));
                    // Attempt to list devices in `/media/$USER`
                    device_paths = UsbInfo::run_ls_command(&media_path).unwrap_or_else(|_| {
                        // If it fails, fallback to `/mnt`
                        let mnt_path = PathBuf::from("/mnt");
                        UsbInfo::run_ls_command(&mnt_path).unwrap_or_else(|_| Vec::new())
                    });
                }
            }
            "macos" => {
                let volumes_path = PathBuf::from("/Volumes");
                device_paths = UsbInfo::run_ls_command(&volumes_path)?;
            }
            _ => {
                return Err("Unsupported operating system".to_string());
            }
        }
        Ok(device_paths)
    }    // Method to find the device path by comparing baseline and new snapshot
    pub fn detect_new_device_path(&mut self, platform: &str) -> Result<Option<PathBuf>, String> {
        let latest_hash: HashSet<PathBuf> = UsbInfo::query_devices(platform)?.into_iter().collect();
        // Find the difference between the two sets
        let differences: Vec<&PathBuf> = latest_hash.difference(&self.baseline).collect();
        // Error if more than one unique path is found
        if differences.len() > 1 {
            return Err("More than one target USB device found".to_string());
        }
        // If exactly 1 device is found, return the new device path
        if let Some(&device) = differences.first() {
            return Ok(Some(device.clone()));
        }
        // If no new device was found, return None
        Ok(None)
    }    // Obtain file path prefix based on platform
    pub fn obtain_file_path_prefix(platform: &str) -> Result<String, String> {
        match platform {
            "windows" => {
                return Ok("".to_string());
            }
            "linux" => {
                if let Some(user) = env::var_os("USER") {
                    let media_path = PathBuf::from(format!("/media/{}/", user.to_string_lossy()));
                    return Ok(media_path.to_string_lossy().into_owned())
                }else{
                    return Err("Error detecting user".to_string())
                }
            }
            "macos" => {
                return Ok("/Volumes/".to_string());
            }
            _ => {
                return Err("Unsupported operating system".to_string());
            }
        }
    }    // Write function to create a file and write a simple payload
    pub fn write_to_file(device_path: &str, payload: &str, platform: &str) -> io::Result<()> {
        println!("writing file");
        let file_path_prefix: String = UsbInfo::obtain_file_path_prefix(platform).unwrap();
        println!("File Path Prefix: {}", file_path_prefix);
        let file_path = format!("{}{}/hello_world", file_path_prefix, device_path);
        println!("File path: {}", file_path);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            // Overwrites if file exists
            .open(file_path)?;
        file.write_all(payload.as_bytes())?;
        Ok(())
    }    // Read function to read the contents of the file if it exists
    pub fn read_from_file(device_path: &str, platform: &str) -> io::Result<String> {
        println!("Reading File");
        let file_path_prefix: String = UsbInfo::obtain_file_path_prefix(platform).unwrap();
        println!("File Path Prefix: {}", file_path_prefix);
        let file_path = format!("{}{}/hello_world", file_path_prefix, device_path);
        println!("File Path: {}", file_path);
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}