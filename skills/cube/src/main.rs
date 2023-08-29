use color_eyre::eyre::Result;
use tracing::info;

fn main() -> Result<()> {
	// Set up nice error messages and logging
	color_eyre::install()?;
	tracing_subscriber::fmt::init();

	info!("Running `cube` skill");

	Ok(())
}
