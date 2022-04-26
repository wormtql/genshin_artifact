#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{Manager, Wry};
use tauri::App;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct YasConfig {
    pub min_star: usize,
}

fn start_yas(config: &YasConfig) {
    let scanner =
}

fn setup_app(app: &mut App<Wry>) {
    let id = app.listen_global("yas-scan", |event| {
        let payload_str = event.payload().unwrap();
        let yas_config: YasConfig = serde_json::from_str(payload_str).unwrap();
        println!("yas config: {:?}", yas_config);
    });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            setup_app(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
