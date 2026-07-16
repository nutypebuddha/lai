//! # LocalBackend — local GGUF model inference via llama-gguf
//!
//! Loads a GGUF model (Qwen2.5-0.5B recommended) using llama-gguf's Engine
//! and runs inference on the local CPU.

use crate::inference::config::InferenceConfig;
use crate::inference::{
    BackendKind, Capability, HealthStatus, InferenceBackend, InferenceError, InferenceRequest,
    InferenceResponse,
};
use std::sync::Mutex;

/// Local inference backend using llama-gguf's Engine.
pub struct LocalBackend {
    config: InferenceConfig,
    engine: Mutex<Option<llama_gguf::Engine>>,
    initialized: bool,
    init_error: Option<String>,
}

impl LocalBackend {
    /// Create a new LocalBackend from config. Does NOT load the model yet.
    pub fn new(config: InferenceConfig) -> Self {
        Self {
            config,
            engine: Mutex::new(None),
            initialized: false,
            init_error: None,
        }
    }

    /// Load the model from the configured path, auto-downloading the pinned
    /// Qwen GGUF into the cache dir if no model is found (see download.rs).
    pub fn load(&mut self) -> Result<(), InferenceError> {
        let model_path = crate::inference::download::ensure_model(&self.config)?;

        if !model_path.exists() {
            return Err(InferenceError::ModelNotLoaded(format!(
                "model file not found: {}",
                model_path.display()
            )));
        }

        let path_str = model_path.to_str().unwrap_or("model.gguf").to_string();

        let engine_config = llama_gguf::EngineConfig {
            model_path: path_str,
            temperature: self.config.temperature as f32,
            top_p: self.config.top_p as f32,
            max_tokens: self.config.max_tokens,
            max_context_len: Some(self.config.context_size),
            use_gpu: false,
            ..Default::default()
        };

        match llama_gguf::Engine::load(engine_config) {
            Ok(engine) => {
                let mut guard = self.engine.lock().unwrap();
                *guard = Some(engine);
                self.initialized = true;
                self.init_error = None;
                Ok(())
            }
            Err(e) => {
                self.initialized = true;
                let msg = format!("failed to load model: {}", e);
                self.init_error = Some(msg.clone());
                Err(InferenceError::ModelNotLoaded(msg))
            }
        }
    }

    /// Generate a response using the loaded model.
    fn generate_inner(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, InferenceError> {
        let guard = self.engine.lock().unwrap();
        let engine = guard.as_ref().ok_or_else(|| {
            InferenceError::ModelNotLoaded("model not loaded. Call load() first.".to_string())
        })?;

        let system = request
            .system_prompt
            .as_deref()
            .unwrap_or(&self.config.system_prompt);

        let max_tokens = request.max_tokens.unwrap_or(self.config.max_tokens);

        // Build a chat-formatted prompt
        // The Engine auto-detects chat templates from GGUF metadata,
        // so we just need to use the model's expected format.
        // If it has a chat template, Engine::generate wraps it automatically.
        let full_prompt = if !system.is_empty() {
            format!(
                "<|im_start|>system\n{}\n<|im_end|>\n<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
                system, request.prompt
            )
        } else {
            format!(
                "<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
                request.prompt
            )
        };

        match engine.generate(&full_prompt, max_tokens) {
            Ok(text) => {
                let tokens_generated = max_tokens.min(text.len() / 2);
                Ok(InferenceResponse {
                    text: text.trim().to_string(),
                    tokens_generated,
                    tokens_per_second: 0.0,
                    finish_reason: "stop".to_string(),
                    raw: Some(text),
                })
            }
            Err(e) => Err(InferenceError::InferenceFailed(format!(
                "generation failed: {}",
                e
            ))),
        }
    }
}

impl InferenceBackend for LocalBackend {
    fn generate(&mut self, request: InferenceRequest) -> Result<InferenceResponse, InferenceError> {
        if !self.initialized {
            self.load()?;
        }
        self.generate_inner(request)
    }

    fn health(&self) -> HealthStatus {
        let guard = self.engine.lock().unwrap();
        let model_loaded = guard.is_some();
        HealthStatus {
            healthy: model_loaded && self.init_error.is_none(),
            model_loaded: if model_loaded {
                self.config.model_path.clone()
            } else {
                None
            },
            backend_kind: BackendKind::Local,
            context_size: Some(self.config.context_size),
            message: if model_loaded {
                "Local backend ready".to_string()
            } else if let Some(ref err) = self.init_error {
                format!("Local backend error: {}", err)
            } else {
                "Local backend not loaded (lazy-load on first use)".to_string()
            },
        }
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability {
            name: "llama-gguf".to_string(),
            version: Some("0.10".to_string()),
            supported_features: vec![
                "local".to_string(),
                "cpu".to_string(),
                "gguf".to_string(),
                "qwen2.5".to_string(),
            ],
        }]
    }

    fn kind(&self) -> BackendKind {
        BackendKind::Local
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inference::config::InferenceConfig;

    #[test]
    fn test_local_backend_health_not_loaded() {
        let config = InferenceConfig::default();
        let backend = LocalBackend::new(config);
        let health = backend.health();
        assert!(!health.healthy);
        assert_eq!(health.backend_kind, BackendKind::Local);
    }

    #[test]
    fn test_local_backend_capabilities() {
        let config = InferenceConfig::default();
        let backend = LocalBackend::new(config);
        let caps = backend.capabilities();
        assert!(!caps.is_empty());
        assert_eq!(caps[0].name, "llama-gguf");
    }
}
