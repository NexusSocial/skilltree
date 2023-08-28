use color_eyre::eyre::Result;
use tracing::info;

fn main() -> Result<()> {
	color_eyre::install()?;
	info!("Hello world!");

	Ok(())
}
