use crate::args::Column;
use crates_io_metadata::scrap::scrap;
use toml_edit::{Key, Value};
use crate::ARGS;

pub async fn parse_dependency(keys: Vec<&Key>, dependency: &Value, columns: &Vec<Column>) -> anyhow::Result<Vec<Vec<String>>> {
    let mut rows = vec![];

    let key = keys.first().unwrap();
    let name = key.to_string();
    let local = match dependency {
        Value::InlineTable(dependency_table) => dependency_table.contains_key("path"),
        _ => false,
    };

    let should_parse_crates_io = !local &&
        columns
        .iter()
        .any(|column| match column {
            Column::NameWithRepository | Column::NameWithDocumentation | Column::NameWithHomepage | Column::NameWithAnyLink => true,
            _ => false,
        });

    let crates_io = match should_parse_crates_io {
        true => Some(scrap(&name).await?),
        false => None
    };

    let decors = get_decors(key);
    let (section, comment) = parse_comments(decors);

    if let Some(section) = section {
        let mut section_row = vec![String::new(); columns.len()];
        section_row[0] = format!("**{section}**");
        rows.push(section_row);
    }

    if comment.is_none() && ARGS.skip_uncommented {
        return Ok(rows);
    }

    let mut crate_row = vec![];

    for column in columns {
        let value = match column {
            Column::Name => name.clone(),
            Column::NameWithRepository => match local {
                true => name.clone(),
                false => match &crates_io.as_ref().unwrap().crate_info.repository {
                    None => name.clone(),
                    Some(repository) => format!("[{}]({})", name, repository)
                }
            },
            Column::NameWithDocumentation => match local {
                true => name.clone(),
                false => match &crates_io.as_ref().unwrap().crate_info.documentation {
                    None => name.clone(),
                    Some(documentation) => format!("[{}]({})", name, documentation)
                }
            }
            Column::NameWithHomepage => match local {
                true => name.clone(),
                false => match &crates_io.as_ref().unwrap().crate_info.homepage {
                    None => name.clone(),
                    Some(homepage) => format!("[{}]({})", name, homepage)
                }
            }
            Column::NameWithAnyLink => match local {
                true => name.clone(),
                false => {
                    let crates_io = &crates_io.as_ref().unwrap();
                    let link = match &crates_io.crate_info.repository {
                        Some(repository) => Some(repository),
                        None => match &crates_io.crate_info.homepage {
                            Some(homepage) => Some(homepage),
                            None => match &crates_io.crate_info.documentation {
                                Some(documentation) => Some(documentation),
                                None => None
                            }
                        }
                    };

                    match link {
                        None => name.clone(),
                        Some(link) => format!("[{}]({})", name, link)
                    }
                }
            }
            Column::Version => get_version(dependency, local, &name)
                .replace("\"", "")
                .replace("=", "")
                .replace("~", ""),
            Column::ExactVersion => get_version(dependency, local, &name),
            Column::Features => get_features(dependency),
            Column::Comment => match &comment {
                Some(comment) => comment.replace("\n", ""),
                None => String::new(),
            }
        };

        crate_row.push(value);
    }

    rows.push(crate_row);

    Ok(rows)
}

fn get_decors(key: &Key) -> Option<String> {
    match (key.leaf_decor().prefix(), key.leaf_decor().suffix()) {
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
    }
}

fn get_version(dependency: &Value, local: bool, name: &str) -> String {
    match dependency {
        Value::InlineTable(dependency_table) => {
            match dependency_table.get("version") {
                None => match local {
                    true => String::from("local path"),
                    false => panic!("Dependency {name} has no \"version\" value")
                },
                Some(version_value) => match version_value {
                    Value::String(version) => version.to_string().trim().to_string(),
                    _ => panic!("Dependency {name} version is not \"String\" type"),
                }
            }
        },
        Value::String(version_value) => version_value.to_string().trim().to_string(),
        _ => panic!("Dependency {name} value is not \"InlineTable\" type"),
    }
}

fn get_features(dependency: &Value) -> String {
    match dependency {
        Value::InlineTable(dependency_table) => match dependency_table.get("features") {
            None => String::new(),
            Some(features_table) => match features_table {
                Value::Array(features) => features
                    .iter()
                    .map(|f| f.to_string().replace("\"", ""))
                    .collect::<Vec<String>>()
                    .join(", "),
                _ => String::new()
            }
        },
        _ => String::new()
    }
}

fn parse_comments(decors: Option<String>) -> (Option<String>, Option<String>) {
    let mut section: Option<String> = None;
    let mut comment: Option<String> = None;

    if let Some(decors) = decors {
        let lines = decors.lines();

        for line in lines {
            if line.starts_with("# ") {
                section = Some(line[2..].trim().to_string());
                continue;
            }
            else {
                let line = line.replace("#", "");

                if !line.starts_with("!") {
                    comment = match comment {
                        None => Some(line.trim().to_string()),
                        Some(reason) => Some(format!("{} {}", reason, line.replace("#", "").trim()))
                    }
                }
            }
        }
    }

    (section, comment)
}