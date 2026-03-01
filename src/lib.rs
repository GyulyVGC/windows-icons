mod utils {
    pub mod image_utils;
    pub mod process_utils;
}
mod dll_icons;
mod uwp_apps;

pub use dll_icons::DllIcon;
use dll_icons::get_dll_hicon_to_image;
use utils::image_utils::{get_hicon_to_image};
use uwp_apps::{get_uwp_icon};

use std::{error::Error, path::Path};


fn is_uwp_app(path: &Path) -> bool {
    let is_uwp = path.to_string_lossy().contains("Program Files\\WindowsApps");

    let is_wsa = path
        .to_string_lossy()
        .contains("WindowsSubsystemForAndroid");

    is_uwp && !is_wsa
}

pub fn get_icon_by_path<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Box<dyn Error>> {
    let path = path.as_ref();
    if is_uwp_app(path) {
        println!("Detected UWP app for path: '{path:?}'");
        get_uwp_icon(path)
    } else {
        get_hicon_to_image(path)
    }
}

pub fn get_icon_by_dll(dll_icon: DllIcon) -> Result<Vec<u8>, Box<dyn Error>> {
    get_dll_hicon_to_image(dll_icon)
}
