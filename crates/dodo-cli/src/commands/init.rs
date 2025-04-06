pub fn handle() -> anyhow::Result<()> {
    println!("Running dodo init...");
    println!("Are your test directories named `tests/`? (y/n)");

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;

    let test_dirs = if answer.trim().to_lowercase() == "y" {
        vec!["tests".into()]
    } else {
        println!("Enter test directories (comma-separated):");
        let mut custom = String::new();
        std::io::stdin().read_line(&mut custom)?;
        custom
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    };

    dodo_core::init::handle(&test_dirs)?;
    Ok(())
}
