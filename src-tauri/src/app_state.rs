use crate::{database::Database, translation::TranslationService};
use std::sync::Mutex;

/// Shared application state registered with Tauri.
pub struct AppState {
    pub db: Mutex<Database>,
    pub translation_service: Mutex<TranslationService>,
}
