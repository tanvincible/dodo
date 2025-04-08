use crate::scanner::controller::scan_with_magika;
use std::sync::Arc;
use dodo_ai::{AiEngine, phi3::Phi3MiniEngine};

pub fn handle(test_dirs: &[String]) -> anyhow::Result<()> {
    let ai_engine: Arc<dyn AiEngine> = Arc::new(Phi3MiniEngine::new()?);
    scan_with_magika(test_dirs)?;
    Ok(())
}
