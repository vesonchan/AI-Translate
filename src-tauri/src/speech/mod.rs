// Speech-to-Text module
// Integrates Whisper-based offline speech recognition

pub mod audio;
pub mod whisper;
pub mod model_manager;

pub use audio::AudioRecorder;
pub use whisper::WhisperEngine;
pub use model_manager::ModelManager;
