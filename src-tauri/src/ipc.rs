pub mod command {
    #[tauri::command]
    pub fn request_duration() -> usize {
        5000
    }

    #[tauri::command]
    pub fn request_default_duration() -> usize {
        5000
    }

    #[tauri::command]
    pub fn request_text_color_style() -> String {
        "rgba(255, 255, 255, 1.0)".to_string()
    }

    #[tauri::command]
    pub fn request_text_stroke_style() -> String {
        "2px rgba(0, 0, 0, 1.0)".to_string()
    }

    #[tauri::command]
    pub fn request_newline_enabled() -> bool {
        true
    }

    #[tauri::command]
    pub fn request_icon_enabled() -> bool {
        true
    }

    #[tauri::command]
    pub fn request_inline_img_enabled() -> bool {
        true
    }

    #[tauri::command]
    pub fn request_img_enabled() -> bool {
        true
    }

    #[tauri::command]
    pub fn request_video_enabled() -> bool {
        true
    }

    #[tauri::command]
    pub fn request_round_icon_enabled() -> bool {
        true
    }
}
