use clap::ArgAction;
use clap::{Parser, ValueEnum};
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    package: Option<String>,

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