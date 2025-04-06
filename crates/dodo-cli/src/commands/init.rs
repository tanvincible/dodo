pub fn handle() -> anyhow::Result<()> {
    println!("Running dodo init...");
    println!("Are your test directories named `tests/`? (Y/n) [default: Y]");

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    let answer = answer.trim().to_lowercase();

    let test_dirs = if answer.is_empty() || answer == "y" {
        vec!["tests".into()]
    } else {
        println!("Enter test directories (space-separated):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    };

    dodo_core::init::handle(&test_dirs)?;
    Ok(())
}
