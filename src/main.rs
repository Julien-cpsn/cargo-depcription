mod args;
mod parse_manifest;

use crate::args::{CargoCli, Column, Depcription};
use clap::Parser;
use once_cell::sync::Lazy;
use std::{env, fs};
use std::borrow::ToOwned;
use std::clone::Clone;
use to_markdown_table::{MarkdownTable, TableRow};
use toml_edit::{DocumentMut, Item};
use crate::parse_manifest::parse_dependency;

pub const ARGS: Lazy<Depcription> = Lazy::new(|| (*CargoCli::parse().as_depcription()).clone());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manifest_path = match &ARGS.manifest_path {
        Some(path) => path.clone(),
        _ => env::current_dir()?.join("Cargo.toml"),
    };

    let columns = match &ARGS.column {
        Some(columns) => columns.clone(),
        None => vec![
            Column::Name,
            Column::Version,
            Column::Comment
        ]
    };

    let manifest_str = fs::read_to_string(manifest_path).expect("Could not read the Cargo.toml file");

    let doc: DocumentMut = manifest_str.parse()?;

    let dependencies_table = match doc.get("dependencies") {
        None => panic!("No dependencies section"),
        Some(dependencies) => match dependencies {
            Item::None => panic!("Dependencies section has \"None\" type"),
            Item::Value(_) => panic!("Dependencies section has \"Value\" type"),
            Item::ArrayOfTables(_) => panic!("Dependencies section has \"ArrayOfTable\" type"),
            Item::Table(dependencies_table) => dependencies_table,
        }
    };

    let mut markdown_dependencies: Vec<TableRow> = vec![];

    for (keys, dependency) in dependencies_table.get_values() {
        let rows = parse_dependency(keys, dependency, &columns).await?;

        let table_rows: Vec<TableRow> = rows
            .iter()
            .map(|row| TableRow::new(row.to_owned()))
            .collect();

        markdown_dependencies.extend(table_rows);
    }

    let headers = columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            if index == 0 {
                format!("**Category / {column}**")
            }
            else {
                format!("**{column}**")
            }
        })
        .collect::<Vec<String>>();

    let table = MarkdownTable::new(
        Some(headers),
        markdown_dependencies
    )?;

    let stringed_table = table
        .to_string()
        .replace("- | -", "--|--")
        .replace("| -", "|--")
        .replace("- |", "--|");

    println!("{stringed_table}");

    Ok(())
}