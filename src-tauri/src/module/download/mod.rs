pub mod dwl_main;
pub mod decompression;
pub mod paths;

pub fn get_user_os() -> String {
    match std::env::consts::OS {
        "windows" => "windows".to_string(),
        "macos" => "osx".to_string(),
        "linux" => "linux".to_string(),
        _ => "unknown".to_string(),
    }
}
