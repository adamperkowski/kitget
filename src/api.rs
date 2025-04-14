//! Main fetching functionality

use crate::error::{Error, Result};
use bytes::Bytes;
use reqwest::StatusCode;

/// Base URL for the Cat as a Service API
const URL: &str = "https://cataas.com/cat";

/// Customization options ([API docs](https://cataas.com/doc.html))
pub struct Options {
    /// Whether to fetch a gif instead of a static image
    ///
    /// `/gif`
    pub gif: bool,
    /// Tags to filter the images by
    ///
    /// `/:tags`
    pub tags: Option<String>,
    /// Text to display on the image
    ///
    /// `/says/:display`
    pub display: Option<String>,
    /// Whether to query only square images
    ///
    /// `?type=square`
    pub square: bool,
    /// Whether to query only monochrome images
    ///
    /// `?filter=mono`
    pub mono: bool,
    /// Width of the image
    ///
    /// `?width=:width`
    pub width: Option<usize>,
    /// Height of the image
    ///
    /// `?height=:height`
    pub height: Option<usize>,
    /// Blur value (0.3 - 1000.0)
    ///
    /// `?blur=:blur`
    pub blur: Option<f32>,
    /// Custom RGB filter values
    ///
    /// `?filter=custom&r=:red&g=:green&b=:blue`
    pub red: Option<u8>,
    /// Custom RGB filter values
    ///
    /// `?filter=custom&r=:red&g=:green&b=:blue`
    pub green: Option<u8>,
    /// Custom RGB filter values
    ///
    /// `?filter=custom&r=:red&g=:green&b=:blue`
    pub blue: Option<u8>,
    /// Brightness value
    ///
    /// `?filter=custom&brightness=:brightness`
    pub brightness: Option<f32>,
    /// Saturation value
    ///
    /// `?filter=custom&saturation=:saturation`
    pub saturation: Option<f32>,
    /// Hue value
    ///
    /// `?filter=custom&hue=:hue`
    pub hue: Option<f32>,
    /// Lightness value
    ///
    /// `?filter=custom&lightness=:lightness`
    pub lightness: Option<f32>,
}

impl Options {
    /// Create a new [`Options`] instance from the command-line arguments struct
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

    /// Build the API request URL
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

/// Fetch the image from the API
pub async fn fetch(options: Options) -> Result<Bytes> {
    let response = reqwest::get(options.build()?).await?;
    handle_status(&response.status())?;
    Ok(response.bytes().await?)
}

/// Handle the response status code
fn handle_status(code: &StatusCode) -> Result<()> {
    match *code {
        StatusCode::OK => Ok(()),
        StatusCode::NOT_FOUND => Err(Error::NotFound),
        _ => Err(Error::Request(code.to_string().to_lowercase())),
    }
}
