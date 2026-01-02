use crate::http_client::{configure_http_client, http_client, validate_http_client};
#[cfg(not(target_os = "macos"))]
use crate::system_tray::show_main_window;
use crate::{
    app_state::AppState,
    database::{AppConfig, TokenLimitConfig, TranslationRecord},
    ocr_tasks::run_ocr_on_image_data,
    platform,
    shortcuts::register_shortcuts,
    token_limits::calculate_text_response_tokens,
    translation::{TranslationRequest, TranslationResult, TranslationService},
};
use serde::Serialize;
use serde_json::Value;
use tauri::Emitter;
use tauri::{AppHandle, Manager, State, Window};

type ScreenshotImage = screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>>;

#[derive(Debug, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub label: String,
}

fn screenshot_to_dynamic_image(image: &ScreenshotImage) -> Result<image::DynamicImage, String> {
    let (width, height) = (image.width(), image.height());
    let data = image.clone().into_vec();
    let rgba = image::RgbaImage::from_raw(width, height, data)
        .ok_or_else(|| "无法转换截图到图像缓冲区".to_string())?;
    Ok(image::DynamicImage::ImageRgba8(rgba))
}

fn encode_image_to_png(image: &ScreenshotImage) -> Result<Vec<u8>, String> {
    use image::ImageFormat;
    use std::io::Cursor;

    let dynamic_image = screenshot_to_dynamic_image(image)?;
    let mut cursor = Cursor::new(Vec::new());
    dynamic_image
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| format!("图片编码失败: {}", e))?;
    Ok(cursor.into_inner())
}

#[tauri::command]
pub async fn translate_text(
    text: String,
    from_language: Option<String>,
    to_language: String,
    service: String,
    state: State<'_, AppState>,
) -> Result<TranslationResult, String> {
    let from_lang_value = from_language.unwrap_or_default();
    let to_lang_value = to_language;

    // 根据前端传入的 service 参数决定使用哪个服务
    // 如果 service 是 "google"，则直接使用 Google 翻译
    // 否则根据数据库配置使用 AI 服务
    let (target_service, api_key, base_url, model_id, token_config): (TranslationService, String, String, String, TokenLimitConfig) = {
        let db = state
            .db
            .lock()
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        let config = db.get_app_config()
            .map_err(|e| format!("获取应用配置失败: {}", e))?
            .ok_or_else(|| "无法获取配置".to_string())?;

        let token_config = config.token_limits.clone();
        
        if service == "google" {
            (
                TranslationService::Google,
                "".to_string(),
                "".to_string(),
                "".to_string(),
                token_config,
            )
        } else {
            let translation_config = config.translation;
            (
                TranslationService::OpenAI,
                translation_config.api_key,
                translation_config.base_url,
                translation_config.model_id,
                token_config,
            )
        }
    };

    let max_tokens = calculate_text_response_tokens(&text, Some(&token_config));
    let request = TranslationRequest {
        text,
        from_lang: from_lang_value,
        to_lang: to_lang_value,
        max_tokens,
    };

    target_service
        .translate(request, &api_key, &base_url, &model_id)
        .await
}

#[tauri::command]
pub async fn save_translation(
    original_text: String,
    translated_text: String,
    from_language: String,
    to_language: String,
    service: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    let record = TranslationRecord {
        id: None,
        original_text: original_text.clone(),
        translated_text: translated_text.clone(),
        service: service.clone(),
        from_language: Some(from_language),
        to_language: Some(to_language),
        created_at: Some(chrono::Utc::now().to_rfc3339()),
    };

    db.save_translation(&record)
        .map(|_| ())
        .map_err(|e| format!("保存翻译记录失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_translation_history(
    limit: Option<i32>,
    offset: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<TranslationRecord>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_translation_history(limit, offset)
        .map_err(|e| format!("获取翻译历史失败: {}", e))
}

#[tauri::command]
pub async fn search_history(
    keyword: String,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<TranslationRecord>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.search_history(&keyword, limit)
        .map_err(|e| format!("搜索历史记录失败: {}", e))
}

#[tauri::command]
pub async fn clear_history(state: State<'_, AppState>) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.clear_history()
        .map_err(|e| format!("清空历史记录失败: {}", e))
}

#[tauri::command]
pub async fn save_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.save_setting(&key, &value)
        .map_err(|e| format!("保存设置失败: {}", e))
}

#[tauri::command]
pub async fn get_setting(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_setting(&key)
        .map_err(|e| format!("获取设置失败: {}", e))
}

#[tauri::command]
pub async fn save_api_key(
    service: String,
    api_key: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.save_api_key(&service, &api_key)
        .map_err(|e| format!("保存 API 密钥失败: {}", e))
}

#[tauri::command]
pub async fn get_api_key(
    service: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_api_key(&service)
        .map_err(|e| format!("获取 API 密钥失败: {}", e))
}

#[tauri::command]
pub async fn save_app_config(config: AppConfig, state: State<'_, AppState>) -> Result<(), String> {
    validate_http_client(Some(&config.proxy)).map_err(|e| format!("验证代理配置失败: {}", e))?;

    {
        let db = state
            .db
            .lock()
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        db.save_app_config(&config)
            .map_err(|e| format!("保存应用配置失败: {}", e))?;
    }

    configure_http_client(Some(&config.proxy)).map_err(|e| format!("应用代理配置失败: {}", e))
}

#[tauri::command]
pub async fn get_app_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_app_config()
        .map_err(|e| format!("获取应用配置失败: {}", e))
        .and_then(|config_opt| config_opt.ok_or_else(|| "应用配置不存在".to_string()))
}

#[tauri::command]
pub async fn reload_shortcuts(app_handle: AppHandle) -> Result<(), String> {
    register_shortcuts(&app_handle);
    Ok(())
}

fn parse_model_value(value: &Value) -> Option<ModelInfo> {
    match value {
        Value::String(text) => Some(ModelInfo {
            id: text.clone(),
            label: text.clone(),
        }),
        Value::Object(map) => {
            let identifier = map
                .get("id")
                .and_then(|v| v.as_str())
                .or_else(|| map.get("model").and_then(|v| v.as_str()))
                .or_else(|| map.get("name").and_then(|v| v.as_str()))
                .or_else(|| map.get("slug").and_then(|v| v.as_str()))?;

            let owner = map
                .get("owned_by")
                .and_then(|v| v.as_str())
                .or_else(|| map.get("organization").and_then(|v| v.as_str()))
                .or_else(|| map.get("provider").and_then(|v| v.as_str()));

            let label = owner
                .map(|o| format!("{} ({})", identifier, o))
                .unwrap_or_else(|| identifier.to_string());

            Some(ModelInfo {
                id: identifier.to_string(),
                label,
            })
        }
        _ => None,
    }
}

fn extract_model_list(payload: &Value) -> Vec<ModelInfo> {
    if let Some(array) = payload.as_array() {
        return array.iter().filter_map(parse_model_value).collect();
    }

    if let Some(obj) = payload.as_object() {
        if let Some(array) = obj.get("data").and_then(|v| v.as_array()) {
            return array.iter().filter_map(parse_model_value).collect();
        }

        if let Some(array) = obj.get("models").and_then(|v| v.as_array()) {
            return array.iter().filter_map(parse_model_value).collect();
        }

        if let Some(nested_obj) = obj.get("data").and_then(|v| v.as_object()) {
            return nested_obj.values().filter_map(parse_model_value).collect();
        }

        let mut collected = Vec::new();
        for value in obj.values() {
            if let Some(array) = value.as_array() {
                for item in array {
                    if let Some(model) = parse_model_value(item) {
                        collected.push(model);
                    }
                }
            } else if let Some(model) = parse_model_value(value) {
                collected.push(model);
            }
        }
        return collected;
    }

    parse_model_value(payload).into_iter().collect()
}

#[tauri::command]
pub async fn fetch_available_models(
    base_url: String,
    api_key: String,
) -> Result<Vec<ModelInfo>, String> {
    let trimmed_base = base_url.trim();
    let trimmed_key = api_key.trim();

    if trimmed_base.is_empty() {
        return Err("Base URL不能为空".to_string());
    }

    if trimmed_key.is_empty() {
        return Err("API Key不能为空".to_string());
    }

    let normalized_base = trimmed_base.trim_end_matches('/');
    let endpoint = format!("{}/models", normalized_base);

    let response = http_client()
        .get(&endpoint)
        .header("Authorization", format!("Bearer {}", trimmed_key))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("请求模型列表失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        let error_msg = if error_body.is_empty() {
            format!("模型接口返回错误: {}", status)
        } else {
            format!("模型接口返回错误: {} - {}", status, error_body)
        };
        return Err(error_msg);
    }

    let payload: Value = response
        .json()
        .await
        .map_err(|e| format!("解析模型列表失败: {}", e))?;

    let models = extract_model_list(&payload);

    if models.is_empty() {
        return Err("未从接口获取到模型列表".to_string());
    }

    Ok(models)
}

#[tauri::command]
pub async fn capture_screen() -> Result<String, String> {
    use base64::Engine;
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    let buffer = encode_image_to_png(&image)?;

    let base64_string = base64::engine::general_purpose::STANDARD.encode(&buffer);

    Ok(base64_string)
}

#[tauri::command]
pub async fn capture_screen_area(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<String, String> {
    use base64::Engine;
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    let mut dynamic_image = screenshot_to_dynamic_image(&image)?;

    let cropped_image = dynamic_image.crop(x, y, width, height);

    let mut cropped_buffer = Vec::new();
    cropped_image
        .write_to(
            &mut std::io::Cursor::new(&mut cropped_buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("编码裁剪图片失败: {}", e))?;

    let base64_string = base64::engine::general_purpose::STANDARD.encode(&cropped_buffer);

    Ok(base64_string)
}

#[tauri::command]
pub async fn start_area_selection(app_handle: AppHandle) -> Result<(), String> {
    platform::start_area_selection(app_handle).await
}

#[tauri::command]
pub async fn set_ocr_result(app_handle: AppHandle, text: String) -> Result<(), String> {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-result", text)
            .map_err(|e| format!("发送OCR结果失败: {}", e))?;
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn submit_area_for_ocr(
    app_handle: AppHandle,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-pending", true)
            .map_err(|e| e.to_string())?;
    }
    show_main_window(&app_handle);

    let app_handle_clone = app_handle.clone();
    state.inner();

    tauri::async_runtime::spawn(async move {
        let state = app_handle_clone.state::<AppState>();

        match capture_area_and_ocr(x, y, width, height, state).await {
            Ok(text) => {
                println!("OCR成功,文本长度: {}, 内容: '{}'", text.len(), text);
                if let Some(main_window) = app_handle_clone.get_webview_window("main") {
                    println!("发送ocr-result事件到主窗口");
                    match main_window.emit("ocr-result", &text) {
                        Ok(_) => println!("ocr-result事件发送成功"),
                        Err(e) => println!("ocr-result事件发送失败: {}", e),
                    }
                } else {
                    println!("无法找到主窗口");
                }
            }
            Err(e) => {
                println!("Background OCR failed: {}", e);
                if let Some(main_window) = app_handle_clone.get_webview_window("main") {
                    let error_msg = format!("Error: {}", e);
                    let _ = main_window.emit("ocr-result", error_msg);
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn capture_area_and_ocr(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let center_x = x + (width as i32 / 2);
    let center_y = y + (height as i32 / 2);

    let screen = screens
        .iter()
        .find(|s| {
            let info = &s.display_info;
            center_x >= info.x
                && center_x < (info.x + info.width as i32)
                && center_y >= info.y
                && center_y < (info.y + info.height as i32)
        })
        .unwrap_or(&screens[0]);

    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    let mut dynamic_image = screenshot_to_dynamic_image(&image)?;

    let relative_x = (x - screen.display_info.x) as u32;
    let relative_y = (y - screen.display_info.y) as u32;

    let cropped_image = dynamic_image.crop(relative_x, relative_y, width, height);

    let mut cropped_buffer = Vec::new();
    cropped_image
        .write_to(
            &mut std::io::Cursor::new(&mut cropped_buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("编码裁剪图片失败: {}", e))?;

    run_ocr_on_image_data(cropped_buffer, state).await
}

#[tauri::command]
pub async fn capture_and_ocr(state: State<'_, AppState>) -> Result<String, String> {
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    let buffer = encode_image_to_png(&image)?;

    run_ocr_on_image_data(buffer, state).await
}

// ============================================================================
// Speech-to-Text Commands
// ============================================================================

use crate::speech::{AudioRecorder, ModelManager, WhisperEngine};
use crate::speech::whisper::WhisperModel;
use crate::speech::model_manager::ModelStatus;
use std::sync::OnceLock;

// Global speech state (using OnceLock for lazy initialization)
static AUDIO_RECORDER: OnceLock<std::sync::Mutex<Option<AudioRecorder>>> = OnceLock::new();
static WHISPER_ENGINE: OnceLock<std::sync::Mutex<WhisperEngine>> = OnceLock::new();
static MODEL_MANAGER: OnceLock<ModelManager> = OnceLock::new();

fn get_audio_recorder() -> &'static std::sync::Mutex<Option<AudioRecorder>> {
    AUDIO_RECORDER.get_or_init(|| std::sync::Mutex::new(None))
}

fn get_whisper_engine() -> &'static std::sync::Mutex<WhisperEngine> {
    WHISPER_ENGINE.get_or_init(|| std::sync::Mutex::new(WhisperEngine::new()))
}

fn get_model_manager() -> &'static ModelManager {
    MODEL_MANAGER.get_or_init(|| ModelManager::new().expect("Failed to create model manager"))
}

/// Get list of all available Whisper models with their download status
#[tauri::command]
pub fn get_speech_models() -> Vec<ModelStatus> {
    get_model_manager().get_all_models_status()
}

/// Check if a specific model is downloaded
#[tauri::command]
pub fn is_model_downloaded(model: String) -> bool {
    let model = match model.as_str() {
        "tiny" => WhisperModel::Tiny,
        "base" => WhisperModel::Base,
        "small" => WhisperModel::Small,
        "medium" => WhisperModel::Medium,
        "turbo" => WhisperModel::Turbo,
        "large" => WhisperModel::Large,
        _ => return false,
    };
    get_model_manager().is_model_available(model)
}

/// Download a Whisper model
#[tauri::command]
pub async fn download_speech_model(
    model: String,
    app: AppHandle,
) -> Result<(), String> {
    let model_enum = match model.as_str() {
        "tiny" => WhisperModel::Tiny,
        "base" => WhisperModel::Base,
        "small" => WhisperModel::Small,
        "medium" => WhisperModel::Medium,
        "turbo" => WhisperModel::Turbo,
        "large" => WhisperModel::Large,
        _ => return Err("Invalid model name".to_string()),
    };

    let manager = get_model_manager();
    let model_name = model.clone();
    
    // Run download in blocking task
    tokio::task::spawn_blocking(move || {
        manager.download_model(model_enum, |downloaded, total| {
            let progress = if total > 0 {
                (downloaded as f64 / total as f64 * 100.0) as u32
            } else {
                0
            };
            let _ = app.emit("speech-model-download-progress", serde_json::json!({
                "model": model_name.clone(),
                "downloaded": downloaded,
                "total": total,
                "progress": progress
            }));
        })
    })
    .await
    .map_err(|e| format!("Download task failed: {}", e))?
    .map_err(|e| format!("Model download failed: {}", e))
}

/// Load a Whisper model for transcription
#[tauri::command]
pub fn load_speech_model(model: String) -> Result<(), String> {
    let model_enum = match model.as_str() {
        "tiny" => WhisperModel::Tiny,
        "base" => WhisperModel::Base,
        "small" => WhisperModel::Small,
        "medium" => WhisperModel::Medium,
        "turbo" => WhisperModel::Turbo,
        "large" => WhisperModel::Large,
        _ => return Err("Invalid model name".to_string()),
    };

    let manager = get_model_manager();
    let model_path = manager.get_model_path(model_enum);

    if !model_path.exists() {
        return Err("Model not downloaded".to_string());
    }

    let mut engine = get_whisper_engine().lock().unwrap();
    engine.load_model(&model_path).map_err(|e| e.to_string())
}

/// Start audio recording
#[tauri::command]
pub fn start_speech_recording(app: AppHandle) -> Result<(), String> {
    println!("DEBUG: start_speech_recording command received");
    let _ = app.emit("backend-alert", "开始初始化录音...");
    
    let mut recorder_opt = get_audio_recorder().lock().unwrap();
    
    // Create recorder if not exists
    if recorder_opt.is_none() {
        println!("DEBUG: Initializing AudioRecorder...");
        let _ = app.emit("backend-alert", "初始化音频设备...");
        match AudioRecorder::new() {
            Ok(recorder) => {
                println!("DEBUG: AudioRecorder initialized successfully");
                let _ = app.emit("backend-alert", "音频设备初始化成功");
                *recorder_opt = Some(recorder);
            }
            Err(e) => {
                println!("DEBUG: Failed to initialize AudioRecorder: {}", e);
                let _ = app.emit("backend-alert", format!("音频设备初始化失败: {}", e));
                return Err(e.to_string());
            }
        }
    }

    if let Some(ref mut recorder) = *recorder_opt {
        println!("DEBUG: Starting recording on recorder instance");
        recorder.start_recording().map_err(|e| {
            println!("DEBUG: Failed to start recording: {}", e);
            let _ = app.emit("backend-alert", format!("启动录音失败: {}", e));
            e.to_string()
        })?;
        let _ = app.emit("backend-alert", "录音已开始！请说话...");
        Ok(())
    } else {
        Err("Failed to initialize recorder".to_string())
    }
}

/// Stop recording and transcribe
#[tauri::command]
pub fn stop_speech_recording() -> Result<String, String> {
    let mut recorder_opt = get_audio_recorder().lock().unwrap();
    
    let samples = if let Some(ref mut recorder) = *recorder_opt {
        recorder.stop_recording().map_err(|e| e.to_string())?
    } else {
        return Err("No active recording".to_string());
    };

    // Transcribe
    let engine = get_whisper_engine().lock().unwrap();
    if !engine.is_loaded() {
        return Err("Whisper model not loaded".to_string());
    }

    engine.transcribe(&samples).map_err(|e| e.to_string())
}

/// Check if Whisper model is loaded
#[tauri::command]
pub fn is_speech_model_loaded() -> bool {
    let engine = get_whisper_engine().lock().unwrap();
    engine.is_loaded()
}

/// Get available audio input devices
#[tauri::command]
pub fn get_audio_devices() -> Result<Vec<String>, String> {
    AudioRecorder::list_devices().map_err(|e| e.to_string())
}

/// Set speech language (use "auto" for auto-detection)
#[tauri::command]
pub fn set_speech_language(language: String) -> Result<(), String> {
    let mut engine = get_whisper_engine().lock().unwrap();
    engine.set_language(&language);
    Ok(())
}
