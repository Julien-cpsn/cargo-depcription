# Cargo depcription

Transforms your rust project dependencies into an explicative dependency choice markdown table!

## Installation

Simply

```shell
cargo install depcription
```

## Usage

In current directory

```shell
cargo depcription
```

With path

```shell
cargo depcription ~/Path/to/my/Cargo.toml
```

## Custom columns

> [!NOTE]
> Please note that default columns are name, version and comment

You can use the provided columns in the order you want!

```shell
cargo depcription --column name-with-any-link --column version --column features --column comment
```

## Example

Input:
```toml
...manifest...

[dependencies]
# Serialization & Deserialization
## Deserialize toml manifests
##! Hidden comment
toml_edit = { version = "0.23.1", features = ["serde"] }
## Parse crates-io metadata
crates-io-metadata = { version = "0.1.0", features = ["scrap"] }

# Output
## Transform dependencies to markdown table
to_markdown_table = "0.1.5"

# Utils
## Generic errors
anyhow = "1.0.98"
## CLI parser
clap = { version = "4.5.41", features = ["derive"] }
## Allow async function usage
tokio = { version = "1.46.1", features = ["rt", "rt-multi-thread", "macros"] }
## Lazy variables
once_cell = "1.21.3"
```

Output:
```markdown
| **Category / Library**              | **Version** | **Comment**                              |
|-------------------------------------|-------------|------------------------------------------|
| **Serialization & Deserialization** |             |                                          |
| toml_edit                           | 0.23.1      | Deserialize toml manifests               |
| crates-io-metadata                  | local path  | Parse crates-io metadata                 |
| **Output**                          |             |                                          |
| to_markdown_table                   | 0.1.5       | Transform dependencies to markdown table |
| **Utils**                           |             |                                          |
| anyhow                              | 1.0.98      | Generic errors                           |
| clap                                | 4.5.41      | CLI parser                               |
| tokio                               | 1.46.1      | Allow async function usage               |
| once_cell                           | 1.21.3      | Lazy variables                           |
```

## Help

```shell
Usage: cargo-depcription [OPTIONS] [MANIFEST_PATH]

Arguments:
  [MANIFEST_PATH]  Cargo manifest to use, leave empty for current directory manifest

Options:
  -s, --skip-uncommented  Do not print the dependencies that are not commented
  -c, --column <COLUMN>   Columns to use instead of the default ones [possible values: name, name-with-repository, name-with-documentation, name-with-homepage, name-with-any-link, version, exact-version, features, comment]
  -h, --help              Print help
```

## Dependencies

| **Category / Library**                                                  | **Version** | **Features**                  | **Comment**                              |
|-------------------------------------------------------------------------|-------------|-------------------------------|------------------------------------------|
| **Serialization & Deserialization**                                     |             |                               |                                          |
| [toml_edit](https://github.com/toml-rs/toml)                            | 0.23.1      | serde                         | Deserialize toml manifests               |
| [crates-io-metadata](https://github.com/Julien-cpsn/crates-io-metadata) | 0.1.0       | scrap                         | Parse crates-io metadata                 |
| **Output**                                                              |             |                               |                                          |
| to_markdown_table                                                       | 0.1.5       |                               | Transform dependencies to markdown table |
| **Utils**                                                               |             |                               |                                          |
| [anyhow](https://github.com/dtolnay/anyhow)                             | 1.0.98      |                               | Generic errors                           |
| [clap](https://github.com/clap-rs/clap)                                 | 4.5.41      | derive                        | CLI parser                               |
| [tokio](https://github.com/tokio-rs/tokio)                              | 1.46.1      | rt,  rt-multi-thread,  macros | Allow async function usage               |
| [once_cell](https://github.com/matklad/once_cell)                       | 1.21.3      |                               | Lazy variables                           |

## License

The MIT license for this project can be seen [here](https://github.com/Julien-cpsn/cargo-depcription/blob/main/LICENSE)