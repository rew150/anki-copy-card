use serde::Serialize;
use tap::Pipe;
use tauri_plugin_http::reqwest::Client;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct RB {
    action: String,
    version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
}

#[tauri::command]
async fn anki_request(port: Option<u16>, action: String, params: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
    let port = port.unwrap_or(8765);
    let url = format!("http://localhost:{port}");
    let body = RB {
        action,
        version: 6,
        params,
    };
    let body = serde_json::to_string(&body).map_err(|e| e.to_string())?;
    println!("{body}");
    Client::new()
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(|e| {
            e.to_string()
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            e.to_string()
        })?
        .pipe(|r| Ok(r))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            anki_request,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
