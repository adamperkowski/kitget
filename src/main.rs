use anyhow::Result;
use image::load_from_memory;
use kitget::api::{Options, fetch};
use kitget::cli::Args;
use viuer::{Config, print};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::get();

    let options = Options::from_args(&args);
    let img = fetch(options).await?;

    if let Some(path) = args.output {
        std::fs::write(&path, &img)?;
        println!("Image saved to {}", path);
    } else {
        let config = Config {
            ..Default::default()
        };
        print(&load_from_memory(&img)?, &config)?;
    }

    Ok(())
}
