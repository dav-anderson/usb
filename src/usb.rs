use std::collections::HashSet;
use std::env;
use std::path::{PathBuf};
use std::process::Command;


    pub struct UsbInfo {
        pub baseline: HashSet<PathBuf>, // Store baseline as a HashSet<PathBuf> objects
    }
    impl UsbInfo {
        // Constructor for UsbInfo that initializes the baseline as a HashSet<PathBuf>
        pub fn new(platform: &str) -> Result<Self, String> {
            Ok(UsbInfo {
                baseline: UsbInfo::query_devices(&platform)?.into_iter().collect(),
            })
        }
        // Parse the command output and convert to Vec<PathBuf>
        fn parse_command_output(output: Vec<u8>) -> Vec<PathBuf> {
            let result = String::from_utf8_lossy(&output);
            result
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| PathBuf::from(line.trim()))
                .collect()
        }
        // Run ls command on a given path
        fn run_ls_command(path: &PathBuf) -> Result<Vec<PathBuf>, String> {
            let output = Command::new("ls")
                .arg(path)
                .output();
            Ok(UsbInfo::parse_command_output(output.unwrap().stdout))
        }
        // Method to query devices and return a vector of PathBufs
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
        }
        // Method to find the device path by comparing baseline and new snapshot
        pub fn detect_new_device_path(&mut self, platform: &str) -> Result<Option<PathBuf>, String> {
            // The baseline is already a HashSet<PathBuf>, no need to convert it
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
        }
    }