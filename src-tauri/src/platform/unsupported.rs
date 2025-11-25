use tauri::AppHandle;

pub async fn start_area_selection(app_handle: AppHandle) -> Result<(), String> {
    let _ = app_handle;
    Err("当前平台暂不支持区域截图".to_string())
}
