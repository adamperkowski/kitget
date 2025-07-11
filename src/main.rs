use image::load_from_memory;
use viuer::{Config, print};

use kitget::{
    api::{Options, fetch},
    cli::Args,
    error::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = run(Args::get()).await {
        e.pretty();
        std::process::exit(1);
    }

    Ok(())
}

async fn run(args: Args) -> Result<()> {
    let options = Options::from_args(&args);
    let img = fetch(options).await?;

    if let Some(path) = args.output {
        std::fs::write(&path, &img)?;
        println!("😼 kitteh saved to {path} 😼");
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
