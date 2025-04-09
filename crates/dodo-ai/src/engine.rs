use anyhow::Result;
use std::path::Path;

pub trait AiEngine: Sync + Send {
    fn process_file_with_magika(&self, path: &Path, magika_output: &str) -> Result<()>;
}
