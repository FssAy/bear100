#![cfg(target_os = "windows")]
#![cfg(target_arch = "x86_64")]

#[macro_use]
extern crate tracing;

mod config;
mod log;
mod winapi;
mod app;

use config::Config;
use app::App;

fn main() {
    log::init();

    let cfg = Config::new();

    let mut app = App::new(&cfg);

    if app.run().is_some() {
        info!("success, bear reward changed to: {}$", cfg.replace_value);
    };

    std::thread::sleep(
        std::time::Duration::from_secs(cfg.timeout_s)
    );
}
