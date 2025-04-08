// dodo-core/src/init.rs
use crate::scanner::controller::scan_with_magika;

pub fn handle(test_dirs: &[String]) -> anyhow::Result<()> {
    scan_with_magika(test_dirs)?;
    Ok(())
}
