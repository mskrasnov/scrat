//! Some consts and global variables

// Meta information
pub const PROGRAM_NAME: &str = "Scrat";
pub const PROGRAM_VER: &str = env!("CARGO_PKG_VERSION");
//pub const CLIENT_SITE: &str = "https://mskrasnov.github.io/scrat/";
pub const CLIENT_SITE: &str = "https://github.com/mskrasnov/scrat";
pub const CLIENT_DOCS_PAGE: &str = "https://mskrasnov.github.io/scrat/docs/";

// Some paths...
/// User config directory
pub const CONF_DIR_PATH: &str = ".config/scrat/";
/// Config for `mastodon-async` crate
pub const REG_CONF_PATH: &str = ".config/scrat/reg.toml";
/// Directory with Scrat stylesheets
pub const STYLES_DIR_PATH: &str = ".config/scrat/styles/";
/// Default dark style
pub const DEF_STYLE_DARK_PATH: &str = ".config/scrat/base16_dark.toml";
/// Default light style
pub const DEF_STYLE_LIGHT_PATH: &str = ".config/scrat/base16_light.toml";
