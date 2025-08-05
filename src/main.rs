mod cli;
mod engine;

fn main() -> anyhow::Result<()> {
    let user_input = cli::run_menu()?;
    engine::generate_from_template(user_input)?;
    Ok(())
}