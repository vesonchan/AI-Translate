// Model manager module
// Handles downloading, updating, and managing Whisper models

use crate::speech::whisper::WhisperModel;
use anyhow::{anyhow, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Model manager for Whisper models
pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new() -> Result<Self> {
        let models_dir = Self::get_models_directory()?;

        // Ensure directory exists
        if !models_dir.exists() {
            fs::create_dir_all(&models_dir)?;
        }

        Ok(Self { models_dir })
    }

    /// Get the models directory path
    fn get_models_directory() -> Result<PathBuf> {
        let app_data = dirs::data_local_dir()
            .ok_or_else(|| anyhow!("Cannot find local data directory"))?;
        Ok(app_data.join("AI-Translate").join("whisper-models"))
    }

    /// Get path for a specific model
    pub fn get_model_path(&self, model: WhisperModel) -> PathBuf {
        self.models_dir.join(model.filename())
    }

    /// Check if a model is downloaded
    pub fn is_model_available(&self, model: WhisperModel) -> bool {
        self.get_model_path(model).exists()
    }

    /// Get list of available (downloaded) models
    pub fn get_available_models(&self) -> Vec<WhisperModel> {
        WhisperModel::all()
            .into_iter()
            .filter(|m| self.is_model_available(*m))
            .collect()
    }

    /// Get list of all models with their status
    pub fn get_all_models_status(&self) -> Vec<ModelStatus> {
        WhisperModel::all()
            .into_iter()
            .map(|model| ModelStatus {
                model,
                is_downloaded: self.is_model_available(model),
                display_name: model.display_name().to_string(),
                size_mb: model.size_mb(),
            })
            .collect()
    }

    /// Download a model (blocking)
    pub fn download_model(&self, model: WhisperModel, progress_callback: impl Fn(u64, u64)) -> Result<()> {
        let url = model.download_url();
        let path = self.get_model_path(model);

        // Create temp file
        let temp_path = path.with_extension("downloading");

        let response = reqwest::blocking::Client::new()
            .get(url)
            .send()
            .map_err(|e| anyhow!("Failed to download model: {}", e))?;

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;

        let mut file = fs::File::create(&temp_path)?;
        let mut content = response;

        // Read in chunks and report progress
        let mut buffer = [0u8; 8192];
        loop {
            use std::io::Read;
            let bytes_read = content.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read as u64;
            progress_callback(downloaded, total_size);
        }

        file.flush()?;
        drop(file);

        // Rename to final path
        fs::rename(&temp_path, &path)?;

        Ok(())
    }

    /// Delete a model
    pub fn delete_model(&self, model: WhisperModel) -> Result<()> {
        let path = self.get_model_path(model);
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    /// Get models directory path
    pub fn models_directory(&self) -> &PathBuf {
        &self.models_dir
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().expect("Failed to create model manager")
    }
}

/// Model status information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelStatus {
    pub model: WhisperModel,
    pub is_downloaded: bool,
    pub display_name: String,
    pub size_mb: u32,
}
