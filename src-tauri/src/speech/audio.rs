// Audio recording and processing module
// Uses cpal for cross-platform audio capture

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, SampleRate, Stream, StreamConfig};
use rubato::{FftFixedIn, Resampler};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const WHISPER_SAMPLE_RATE: u32 = 16000;

/// Audio recorder for capturing microphone input
pub struct AudioRecorder {
    device: Device,
    config: StreamConfig,
    is_recording: Arc<AtomicBool>,
    audio_buffer: Arc<Mutex<Vec<f32>>>,
    stream: Option<Stream>,
}

impl AudioRecorder {
    /// Create a new audio recorder with the default input device
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("No input device available"))?;

        let default_config = device.default_input_config()?;
        let config = StreamConfig {
            channels: 1, // Mono for Whisper
            sample_rate: default_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        Ok(Self {
            device,
            config,
            is_recording: Arc::new(AtomicBool::new(false)),
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
            stream: None,
        })
    }

    /// Get list of available input devices
    pub fn list_devices() -> Result<Vec<String>> {
        let host = cpal::default_host();
        let devices: Vec<String> = host
            .input_devices()?
            .filter_map(|d| d.name().ok())
            .collect();
        Ok(devices)
    }

    /// Start recording audio
    pub fn start_recording(&mut self) -> Result<()> {
        if self.is_recording.load(Ordering::SeqCst) {
            return Err(anyhow!("Already recording"));
        }

        // Clear previous buffer
        {
            let mut buffer = self.audio_buffer.lock().unwrap();
            buffer.clear();
        }

        let audio_buffer = Arc::clone(&self.audio_buffer);
        let is_recording = Arc::clone(&self.is_recording);
        let sample_format = self.device.default_input_config()?.sample_format();

        let stream = match sample_format {
            SampleFormat::F32 => self.build_stream::<f32>(audio_buffer, is_recording)?,
            SampleFormat::I16 => self.build_stream::<i16>(audio_buffer, is_recording)?,
            SampleFormat::U16 => self.build_stream::<u16>(audio_buffer, is_recording)?,
            _ => return Err(anyhow!("Unsupported sample format")),
        };

        stream.play()?;
        self.stream = Some(stream);
        self.is_recording.store(true, Ordering::SeqCst);

        Ok(())
    }

    /// Stop recording and return the captured audio as f32 samples at 16kHz
    pub fn stop_recording(&mut self) -> Result<Vec<f32>> {
        if !self.is_recording.load(Ordering::SeqCst) {
            return Err(anyhow!("Not recording"));
        }

        self.is_recording.store(false, Ordering::SeqCst);
        self.stream = None; // Drop the stream to stop recording

        let samples = {
            let buffer = self.audio_buffer.lock().unwrap();
            buffer.clone()
        };

        // Resample to 16kHz if necessary
        let source_rate = self.config.sample_rate.0;
        if source_rate != WHISPER_SAMPLE_RATE {
            self.resample(&samples, source_rate, WHISPER_SAMPLE_RATE)
        } else {
            Ok(samples)
        }
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    fn build_stream<T>(
        &self,
        audio_buffer: Arc<Mutex<Vec<f32>>>,
        is_recording: Arc<AtomicBool>,
    ) -> Result<Stream>
    where
        T: cpal::Sample + cpal::SizedSample + Into<f32>,
    {
        let config = self.config.clone();
        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let stream = self.device.build_input_stream(
            &config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                if is_recording.load(Ordering::SeqCst) {
                    let mut buffer = audio_buffer.lock().unwrap();
                    for &sample in data {
                        buffer.push(sample.into());
                    }
                }
            },
            err_fn,
            None,
        )?;

        Ok(stream)
    }

    fn resample(&self, samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
        let mut resampler = FftFixedIn::<f32>::new(
            from_rate as usize,
            to_rate as usize,
            samples.len(),
            1, // sub_chunks
            1, // channels
        )?;

        let input = vec![samples.to_vec()];
        let output = resampler.process(&input, None)?;

        Ok(output.into_iter().flatten().collect())
    }
}

impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new().expect("Failed to create audio recorder")
    }
}
