use anyhow::Result;
use ignore::{WalkBuilder, gitignore::{Gitignore, GitignoreBuilder}};
use std::path::{Path, PathBuf};

/// Builds a Gitignore matcher that includes:
/// - All nested `.gitignore` files automatically
/// - Custom test directories passed by the user
///
/// This matcher is used to filter out files for scanning.
pub fn build_ignore_matcher(test_dirs: &[String]) -> Result<Gitignore> {
    let mut builder = GitignoreBuilder::new(".");

    // Inject manual ignores like test directories
    for dir in test_dirs {
        builder.add_line(None, &format!("{}/", dir))?;
    }

    // Let the `ignore` crate handle the actual traversal with nested rules
    let matcher = builder.build()?;
    Ok(matcher)
}
