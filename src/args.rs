use std::fmt::Display;
use std::path::PathBuf;
use clap::{ArgAction};
use clap::{Parser, ValueEnum};

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CLAP_STYLING)]
pub enum CargoCli {
    /// Transforms your rust project dependencies into an explicative dependency choice markdown table!
    Depcription(Depcription)
}

impl CargoCli {
    pub fn as_depcription(&self) -> &Depcription {
        match self { CargoCli::Depcription(depcription) => depcription }
    }
}

#[derive(clap::Args, Clone)]
#[command(version, about, long_about = None)]
pub struct Depcription {
    /// Cargo manifest to use, leave empty for current directory manifest
    #[clap(short, long)]
    pub manifest_path: Option<PathBuf>,

    /// Do not print the dependencies that are not commented
    #[clap(short, long, default_value_t = false)]
    pub skip_uncommented: bool,

    /// Columns to use instead of the default ones
    #[clap(short, long, action = ArgAction::Append)]
    pub column: Option<Vec<Column>>
}

#[derive(ValueEnum, Clone)]
pub enum Column {
    Name,
    NameWithRepository,
    NameWithDocumentation,
    NameWithHomepage,
    NameWithAnyLink,
    Version,
    ExactVersion,
    Features,
    Comment
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Column::Name | Column::NameWithRepository | Column::NameWithDocumentation | Column::NameWithHomepage | Column::NameWithAnyLink => String::from("Library"),
            Column::Version => String::from("Version"),
            Column::ExactVersion => String::from("Version"),
            Column::Features => String::from("Features"),
            Column::Comment => String::from("Comment"),
        };
        write!(f, "{}", str)
    }
}