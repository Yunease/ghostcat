mod ollama;

use serde::Serialize;

#[derive(Serialize)]
struct CatResponse {
    reply: String,
    emotion: String,
}

#[tauri::command]
async fn chat_with_cat(message: String) -> Result<CatResponse, String> {
    let (reply, emotion) = ollama::chat_with_cat_impl(&message).await?;
    Ok(CatResponse { reply, emotion })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat_with_cat])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
