use anyhow::{Result, anyhow};
use bytes::Bytes;
use reqwest::StatusCode;

const URL: &str = "https://cataas.com/cat";

pub struct Options {
    gif: bool,
    tags: Option<String>,
    display: Option<String>,
    square: bool,
    mono: bool,
    width: Option<usize>,
    height: Option<usize>,
    blur: Option<f32>,
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
    brightness: Option<f32>,
    saturation: Option<f32>,
    hue: Option<f32>,
    lightness: Option<f32>,
}

impl Options {
    pub fn from_args(args: &crate::cli::Args) -> Self {
        Self {
            gif: args.gif,
            tags: args.tags.clone(),
            display: args.display.clone(),
            square: args.square,
            mono: args.mono,
            width: args.width,
            height: args.height,
            blur: args.blur,
            red: args.red,
            green: args.green,
            blue: args.blue,
            brightness: args.brightness,
            saturation: args.saturation,
            hue: args.hue,
            lightness: args.lightness,
        }
    }

    pub fn build(self) -> Result<String> {
        let mut url = String::from(URL);

        if self.gif {
            url.push_str("/gif");
        }

        if let Some(tags) = self.tags {
            if self.gif {
                return Err(anyhow!("GIFs do not support tags"));
            }

            url.push_str(&format!("/{}", tags));
        }
        if let Some(display) = self.display {
            url.push_str(&format!("/says/{}", display));
        }

        url.push_str("?position=center");

        if self.square {
            url.push_str("&type=square");
        }
        if self.mono {
            if self.red.is_some() || self.green.is_some() || self.blue.is_some() {
                return Err(anyhow!("Monochrome does not support color (duhh)"));
            }

            url.push_str("&filter=mono");
        }

        if let Some(width) = self.width {
            url.push_str(&format!("&width={}", width));
        }
        if let Some(height) = self.height {
            url.push_str(&format!("&height={}", height));
        }
        if let Some(blur) = self.blur {
            if blur < 0.3 || blur > 1000.0 {
                return Err(anyhow!("Blur must be between 0.3 and 1000"));
            }

            url.push_str(&format!("&blur={}", blur));
        }

        if self.red.is_some()
            || self.green.is_some()
            || self.blue.is_some()
            || self.brightness.is_some()
            || self.saturation.is_some()
            || self.hue.is_some()
            || self.lightness.is_some()
        {
            url.push_str("&filter=custom");
        }

        if let Some(red) = self.red {
            if self.green.is_none() || self.blue.is_none() {
                return Err(anyhow!("Red, green, and blue must be set together"));
            }

            url.push_str(&format!("&r={}", red));
        }
        if let Some(green) = self.green {
            if self.red.is_none() || self.blue.is_none() {
                return Err(anyhow!("Red, green, and blue must be set together"));
            }

            url.push_str(&format!("&g={}", green));
        }
        if let Some(blue) = self.blue {
            if self.red.is_none() || self.green.is_none() {
                return Err(anyhow!("Red, green, and blue must be set together"));
            }

            url.push_str(&format!("&b={}", blue));
        }

        if let Some(brightness) = self.brightness {
            url.push_str(&format!("&brightness={}", brightness));
        }
        if let Some(saturation) = self.saturation {
            url.push_str(&format!("&saturation={}", saturation));
        }
        if let Some(hue) = self.hue {
            url.push_str(&format!("&hue={}", hue));
        }
        if let Some(lightness) = self.lightness {
            url.push_str(&format!("&lightness={}", lightness));
        }

        Ok(url)
    }
}

pub async fn fetch(options: Options) -> Result<Bytes> {
    let response = reqwest::get(options.build()?).await?;
    handle_status(&response.status())?;
    Ok(response.bytes().await?)
}

fn handle_status(code: &StatusCode) -> Result<()> {
    match *code {
        StatusCode::OK => Ok(()),
        StatusCode::BAD_REQUEST => Err(anyhow!("Bad request")),
        StatusCode::NOT_FOUND => Err(anyhow!("kitteh not found :(")),
        StatusCode::INTERNAL_SERVER_ERROR => Err(anyhow!("Internal server error")),
        StatusCode::BAD_GATEWAY => Err(anyhow!("Bad gateway")),
        StatusCode::SERVICE_UNAVAILABLE => Err(anyhow!("Service unavailable")),
        StatusCode::TOO_MANY_REQUESTS => Err(anyhow!("Too many requests")),
        _ => Err(anyhow!("Fetching: {}", code.to_string())),
    }
}
