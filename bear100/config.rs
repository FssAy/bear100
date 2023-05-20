use std::io::{Read, Write};

/// Name of the config file
const CONFIG_FILE_NAME: &'static str = "config.json";

/// Type that game uses to hold the bear reward
/// # Notes
/// Might not be exactly right but is 100% safe to modify
pub type BearReward = u32;


#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Config {
    /// Full name of the NFS Unbound process.
    pub process_name: String,

    /// List of all offsets to the "reward" value.
    ///
    /// Order: following `[1; 2; 3; (...)]`
    pub offsets: Vec<u32>,

    /// New "reward" value.
    ///
    /// Amount of money granted for collecting the bear.
    pub replace_value: BearReward,

    /// Time in seconds to wait before exit.
    pub timeout_s: u64,

    /// Time in milliseconds to wait between process list refresh operation.
    pub refresh_rate_ms: u64,

    /// Should the program show retrieved information about the target process.
    pub show_app_info: bool,
}

impl Config {
    /// Tries to load the config file.
    /// # Notes
    /// In case of any failure creates a new default config
    /// and tries to save it on the disk
    pub fn new() -> Config {
        match Config::load() {
            Ok(config) => config,
            Err(msg) => {
                error!("{}", msg);

                warn!("loading the default config");
                let config = Config::default();
                let _ = config.save();  // ignore the save result
                config
            }
        }
    }
}

impl Config {
    /// Map the io error into more user-friendly message
    fn e_io_map(e: std::io::Error) -> String {
        format!("unable to read config file ({})", e)
    }

    /// Map the json error into more user-friendly message
    fn e_json_map(e: serde_json::Error) -> String {
        format!("config file has invalid syntax ({})", e)
    }

    /// Try to load the config file
    fn load() -> Result<Config, String> {
        let mut buffer = String::new();

        std::fs::File::open(CONFIG_FILE_NAME)
            .map_err(Config::e_io_map)?
            .read_to_string(&mut buffer)
            .map_err(Config::e_io_map)?;

        serde_json::from_str::<Config>(&buffer)
            .map_err(Config::e_json_map)
    }

    /// Save the current config
    /// # Notes
    /// Called only when loading the config file failed
    fn save(&self) -> Option<()> {
        let json = serde_json::to_string_pretty(&self)
            .unwrap()
            .into_bytes();

        std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(CONFIG_FILE_NAME)
            .ok()?
            .write_all(&json)
            .ok()
    }
}

impl Default for Config {
    /// The default config for the specific game version (see README.md)
    fn default() -> Self {
        Self {
            process_name: "NeedForSpeedUnbound.exe".to_string(),
            offsets: vec![0x5035070, 0x8, 0x3C0, 0x20, 0x74],
            replace_value: 150_000,
            timeout_s: 4,
            refresh_rate_ms: 700,
            show_app_info: false,
        }
    }
}
