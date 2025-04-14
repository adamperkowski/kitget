use crate::error::{Error, Result};
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
                return Err(Error::GIFsNoTags);
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
                return Err(Error::MonoColors);
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
            if !(0.3..=1000.0).contains(&blur) {
                return Err(Error::BlurValue);
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
                return Err(Error::RGBColors);
            }

            url.push_str(&format!("&r={}", red));
        }
        if let Some(green) = self.green {
            if self.red.is_none() || self.blue.is_none() {
                return Err(Error::RGBColors);
            }

            url.push_str(&format!("&g={}", green));
        }
        if let Some(blue) = self.blue {
            if self.red.is_none() || self.green.is_none() {
                return Err(Error::RGBColors);
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
        StatusCode::NOT_FOUND => Err(Error::NotFound),
        _ => Err(Error::Request(code.to_string().to_lowercase())),
    }
}
