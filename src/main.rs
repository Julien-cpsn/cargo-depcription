use std::{env, fs};
use std::path::PathBuf;
use std::process::exit;
use to_markdown_table::{MarkdownTable, TableRow};
use toml_edit::{DocumentMut, Item, Table, Value};

fn main() {
    let args: Vec<String> = env::args().into_iter().collect();

    let mut skip_uncommented = false;

    for arg in &args[1..] {
        if !arg.starts_with("-") && !arg.starts_with("--") {
            continue;
        }

        match arg.as_str() {
            "--skip-uncommented" => skip_uncommented = true,
            "-h" | "--help" => {
                println!("Cargo depcription\n\n\t--skip-uncommented\tDo not print the dependencies that are not commented");
                exit(0);
            },
            _ => panic!("Unknown argument: {}", arg)
        }
    }

    let manifest_path = match args.get(1) {
        Some(path) if !path.starts_with("-") => PathBuf::from(path),
        _ => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    };

    let manifest_str = fs::read_to_string(manifest_path).expect("Could not read the Cargo.toml file");

    let doc: DocumentMut = manifest_str.parse().unwrap();

    match doc.get("dependencies") {
        None => panic!("No dependencies section"),
        Some(dependencies) => match dependencies {
            Item::None => panic!("Dependencies section has \"None\" type"),
            Item::Value(_) => panic!("Dependencies section has \"Value\" type"),
            Item::ArrayOfTables(_) => panic!("Dependencies section has \"ArrayOfTable\" type"),
            Item::Table(dependencies_table) => dependencies_to_md_table(dependencies_table, skip_uncommented),
        }
    };
}

fn dependencies_to_md_table(dependencies: &Table, skip_uncommented: bool) {
    let mut markdown_dependencies: Vec<TableRow> = vec![];

    for (keys, dependency) in dependencies.get_values() {
        let key = keys.first().unwrap();
        let name = key.to_string();

        let mut version = match dependency {
            Value::InlineTable(dependency_table) => match dependency_table.get("version") {
                None => panic!("Dependency {name} has no \"version\" value"),
                Some(version_value) => match version_value {
                    Value::String(version) => version.to_string(),
                    _ => panic!("Dependency {name} version is not \"String\" type"),
                }
            },
            Value::String(version_value) => version_value.to_string(),
            _ => panic!("Dependency {name} value is not \"InlineTable\" type"),
        };

        version = version
            .replace("\"", "")
            .replace("=", "")
            .replace("~", "");

        let decors = match (key.leaf_decor().prefix(), key.leaf_decor().suffix()) {
            (None, None) => None,
            (Some(prefix), None) => Some(prefix.as_str().unwrap().trim().to_string()),
            (None, Some(suffix)) => Some(suffix.as_str().unwrap().trim().to_string()),
            (Some(prefix), Some(suffix)) => {
                let prefix = prefix.as_str().unwrap().trim();
                let suffix = suffix.as_str().unwrap().trim();

                match (prefix.is_empty(), suffix.is_empty()) {
                    (true, true) => None,
                    (true, false) => Some(suffix.to_string()),
                    (false, true) => Some(prefix.to_string()),
                    (false, false) => Some(format!("{}\n{}", prefix, prefix))
                }
            }
        };

        let mut section_row: Option<TableRow> = None;
        let mut reason: Option<String> = None;
        if let Some(decors) = decors {
            let lines = decors.lines();

            for line in lines {
                if line.starts_with("# ") {
                    section_row = Some(TableRow::new(vec![
                        format!("**{}**", &line[2..]),
                        String::new(),
                        String::new()]
                    ));
                    continue;
                }
                else {
                    let line = line.replace("#", "");
                    reason = match reason {
                        None => Some(line),
                        Some(reason) => Some(format!("{}\n{}", reason, &line.replace("#", "")))
                    }
                }
            }
        }

        if reason.is_none() {
            if skip_uncommented {
                continue;
            }
        }

        let reason = reason.unwrap_or_else(String::new).replace("\n", "");

        if let Some(section_row) = section_row {
            markdown_dependencies.push(section_row);
        }

        markdown_dependencies.push(TableRow::new(vec![
            name,
            version,
            reason
        ]));
    }

    let table = MarkdownTable::new(
        Some(vec![
            String::from("**Category / Library**"),
            String::from("**Version**"),
            String::from("**Reason**")
        ]),
        markdown_dependencies
    ).unwrap();

    let stringed_table = table
        .to_string()
        .replace("- | -", "--|--")
        .replace("| -", "|--")
        .replace("- |", "--|");

    println!("{stringed_table}");
}