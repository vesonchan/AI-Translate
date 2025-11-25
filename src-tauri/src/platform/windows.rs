use screenshots::Screen;
use tauri::{AppHandle, WebviewUrl};

pub async fn start_area_selection(app_handle: AppHandle) -> Result<(), String> {
    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for screen in &screens {
        let info = &screen.display_info;
        min_x = min_x.min(info.x);
        min_y = min_y.min(info.y);
        max_x = max_x.max(info.x + info.width as i32);
        max_y = max_y.max(info.y + info.height as i32);
    }

    let width = (max_x - min_x) as f64;
    let height = (max_y - min_y) as f64;

    let window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "area-selector",
        WebviewUrl::App("area-selector.html".into()),
    )
    .title("区域选择")
    .position(min_x as f64, min_y as f64)
    .inner_size(width, height)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .transparent(true)
    .visible(false)
    .build()
    .map_err(|e| format!("创建区域选择窗口失败: {}", e))?;

    window
        .show()
        .map_err(|e| format!("显示区域选择窗口失败: {}", e))?;
    window
        .set_focus()
        .map_err(|e| format!("设置区域选择窗口焦点失败: {}", e))?;

    Ok(())
}
