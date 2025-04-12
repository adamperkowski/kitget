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
            width: args.x,
            height: args.y,
            x: args.column.unwrap_or(0),
            y: args.row.unwrap_or(0),
            ..Default::default()
        };
        print(&load_from_memory(&img)?, &config)?;
    }

    Ok(())
}
