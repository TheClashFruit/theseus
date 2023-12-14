use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Theme {
  pub id: String,
  pub name: String,
  pub colors: ThemeColors,
  pub font: ThemeFont
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThemeColors {
  pub bg: String,
  pub raised_bg: String,
  pub super_raised_bg: String,
  pub button: String,
  pub base: String,
  pub contrast: String,
  pub accent_contrast: String,
  pub red: String,
  pub orange: String,
  pub green: String,
  pub blue: String,
  pub purple: String,
  pub gray: String,
  pub ad: String,
  pub ad_raised: String,
  pub brand: String,
  pub brand_highlight: String,
  pub brand_shadow: String,
  pub shadow_inset_lg: String,
  pub shadow_inset: String,
  pub shadow_inset_sm: String,
  pub shadow_raised_lg: String,
  pub shadow_raised: String,
  pub shadow_floating: String,
  pub shadow_card: String,
  pub tooltip_text: String,
  pub tooltip_bg: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThemeFont {
  pub standard: String,
  pub mono: String
}

impl Theme {
  #[tracing::instrument]
  pub async fn init(file: &Path) /*-> crate::Result<Self>*/ {

  }
}