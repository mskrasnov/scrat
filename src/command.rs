//! Command parser
//!
//! ## Syntax
//!
//! ```no-test
//! :<COMMAND> <SUBCOMMAND> <ARG 1> <ARG 2> ... <ARG N>
//! ```

#[derive(Debug, Clone)]
pub enum MasterCommand {
    Home,
    PostNew,
    DeletePost,
}

#[derive(Debug, Clone)]
pub struct PostNewCommand {
    pub status: String,
    pub sensitive: Option<bool>,
    pub language: Option
}
