// Whisper inference engine module
// Uses whisper-rs for local speech-to-text

use anyhow::{anyhow, Result};
use std::path::PathBuf;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Supported Whisper model variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WhisperModel {
    Tiny,
    Base,
    Small,
    Medium,
    Turbo,
    Large,
}

impl WhisperModel {
    /// Get the model filename
    pub fn filename(&self) -> &'static str {
        match self {
            WhisperModel::Tiny => "ggml-tiny.bin",
            WhisperModel::Base => "ggml-base.bin",
            WhisperModel::Small => "ggml-small.bin",
            WhisperModel::Medium => "ggml-medium.bin",
            WhisperModel::Turbo => "ggml-large-v3-turbo.bin",
            WhisperModel::Large => "ggml-large-v3.bin",
        }
    }

    /// Get the Hugging Face model URL
    pub fn download_url(&self) -> &'static str {
        match self {
            WhisperModel::Tiny => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
            WhisperModel::Base => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
            WhisperModel::Small => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
            WhisperModel::Medium => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
            WhisperModel::Turbo => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo.bin",
            WhisperModel::Large => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3.bin",
        }
    }

    /// Get approximate model size in MB
    pub fn size_mb(&self) -> u32 {
        match self {
            WhisperModel::Tiny => 75,
            WhisperModel::Base => 142,
            WhisperModel::Small => 466,
            WhisperModel::Medium => 1500,
            WhisperModel::Turbo => 809,
            WhisperModel::Large => 3100,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            WhisperModel::Tiny => "Tiny (75MB)",
            WhisperModel::Base => "Base (142MB)",
            WhisperModel::Small => "Small (466MB)",
            WhisperModel::Medium => "Medium (1.5GB)",
            WhisperModel::Turbo => "Turbo (809MB) - 推荐",
            WhisperModel::Large => "Large (3.1GB)",
        }
    }

    /// List all available models
    pub fn all() -> Vec<WhisperModel> {
        vec![
            WhisperModel::Tiny,
            WhisperModel::Base,
            WhisperModel::Small,
            WhisperModel::Medium,
            WhisperModel::Turbo,
            WhisperModel::Large,
        ]
    }
}

impl Default for WhisperModel {
    fn default() -> Self {
        WhisperModel::Turbo
    }
}

/// Whisper inference engine for speech-to-text
pub struct WhisperEngine {
    context: Option<WhisperContext>,
    current_model: WhisperModel,
    language: String,
}

impl WhisperEngine {
    /// Create a new Whisper engine (model not loaded yet)
    pub fn new() -> Self {
        Self {
            context: None,
            current_model: WhisperModel::default(),
            language: "auto".to_string(),
        }
    }

    /// Load a Whisper model from the given path
    pub fn load_model(&mut self, model_path: &PathBuf) -> Result<()> {
        if !model_path.exists() {
            return Err(anyhow!("Model file not found: {:?}", model_path));
        }

        let params = WhisperContextParameters::default();
        let context = WhisperContext::new_with_params(
            model_path.to_str().ok_or_else(|| anyhow!("Invalid path"))?,
            params,
        )
        .map_err(|e| anyhow!("Failed to load Whisper model: {:?}", e))?;

        self.context = Some(context);
        Ok(())
    }

    /// Transcribe audio samples to text
    /// Input: f32 samples at 16kHz mono
    pub fn transcribe(&self, samples: &[f32]) -> Result<String> {
        let context = self
            .context
            .as_ref()
            .ok_or_else(|| anyhow!("Model not loaded"))?;

        let mut state = context
            .create_state()
            .map_err(|e| anyhow!("Failed to create state: {:?}", e))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // Configure language
        if self.language == "auto" {
            params.set_detect_language(true);
        } else {
            params.set_language(Some(&self.language));
        }

        // Disable timestamps for cleaner output
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // Run inference
        state
            .full(params, samples)
            .map_err(|e| anyhow!("Transcription failed: {:?}", e))?;

        // Collect results
        let num_segments = state.full_n_segments()?;
        let mut text = String::new();

        for i in 0..num_segments {
            if let Ok(segment) = state.full_get_segment_text(i) {
                text.push_str(&segment);
            }
        }

        Ok(text.trim().to_string())
    }

    /// Set the language for transcription
    /// Use "auto" for automatic detection
    pub fn set_language(&mut self, language: &str) {
        self.language = language.to_string();
    }

    /// Get current model
    pub fn current_model(&self) -> WhisperModel {
        self.current_model
    }

    /// Check if a model is loaded
    pub fn is_loaded(&self) -> bool {
        self.context.is_some()
    }
}

impl Default for WhisperEngine {
    fn default() -> Self {
        Self::new()
    }
}
