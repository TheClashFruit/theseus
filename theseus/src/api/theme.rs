use serde::{Deserialize, Serialize};
use crate::state::Theme;

pub async fn get_themes() {

}

pub async fn theme_to_css(theme: Theme) -> String {
  let mut theme_css = "body {".to_string();

  theme_css += &*format!("--color-bg: {};", escape_var(theme.colors.bg));
  theme_css += &*format!("--color-raised-bg: {};", escape_var(theme.colors.raised_bg));
  theme_css += &*format!("--color-super-raised-bg: {};", escape_var(theme.colors.super_raised_bg));
  theme_css += &*format!("--color-button: {};", escape_var(theme.colors.button));
  theme_css += &*format!("--color-base: {};", escape_var(theme.colors.base));
  theme_css += &*format!("--color-contrast: {};", escape_var(theme.colors.contrast));
  theme_css += &*format!("--color-accent-contrast: {};", escape_var(theme.colors.accent_contrast));
  theme_css += &*format!("--color-red: {};", escape_var(theme.colors.red));
  theme_css += &*format!("--color-orange: {};", escape_var(theme.colors.orange));
  theme_css += &*format!("--color-green: {};", escape_var(theme.colors.green));
  theme_css += &*format!("--color-blue: {};", escape_var(theme.colors.blue));
  theme_css += &*format!("--color-purple: {};", escape_var(theme.colors.purple));
  theme_css += &*format!("--color-gray: {};", escape_var(theme.colors.gray));
  theme_css += &*format!("--color-ad: {};", escape_var(theme.colors.ad));
  theme_css += &*format!("--color-ad-raised: {};", escape_var(theme.colors.ad_raised));
  theme_css += &*format!("--color-brand: {};", escape_var(theme.colors.brand));
  theme_css += &*format!("--color-brand-highlight: {};", escape_var(theme.colors.brand_highlight));
  theme_css += &*format!("--color-brand-shadow: {};", escape_var(theme.colors.brand_shadow));
  theme_css += &*format!("--color-shadow-inset-lg: {};", escape_var(theme.colors.shadow_inset_lg));
  theme_css += &*format!("--color-shadow-inset: {};", escape_var(theme.colors.shadow_inset));
  theme_css += &*format!("--color-shadow-inset-sm: {};", escape_var(theme.colors.shadow_inset_sm));
  theme_css += &*format!("--color-shadow-raised-lg: {};", escape_var(theme.colors.shadow_raised_lg));
  theme_css += &*format!("--color-shadow-raised: {};", escape_var(theme.colors.shadow_raised));
  theme_css += &*format!("--color-shadow-floating: {};", escape_var(theme.colors.shadow_floating));
  theme_css += &*format!("--color-shadow-card: {};", escape_var(theme.colors.shadow_card));
  theme_css += &*format!("--color-tooltip-text: {};", escape_var(theme.colors.tooltip_text));
  theme_css += &*format!("--color-tooltip-bg: {};", escape_var(theme.colors.tooltip_bg));

  theme_css += &*format!("--font-standard: {};", escape_var(theme.font.standard));
  theme_css += &*format!("--font-mono: {};", escape_var(theme.font.mono));

  theme_css += "}";

  return theme_css.clone();
}

fn escape_var(string: String) -> String {
  return string
    .replace("<", "")
    .replace(">", "")
    .replace(":", "")
    .replace(";", "");
}