/// Represents the result of file type detection.
#[derive(Debug, Clone)]
pub struct FiletypeResult {
    pub file_path: std::path::PathBuf,
    pub mime: String,
    pub confidence: f32, // Optional: confidence/probability
    pub source: String,  // e.g., "Magika", "CustomAI"
}

/// Trait to be implemented by all file detection engines.
pub trait FiletypeEngine: Send + Sync {
    /// Runs detection on a file, returning a `FiletypeResult`.
    fn detect(&self, file_path: &std::path::Path) -> anyhow::Result<FiletypeResult>;

    /// Human-readable name of the engine (e.g., "Magika")
    fn name(&self) -> &'static str;
}
