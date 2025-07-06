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

## Example

Input:
```toml
...manifest...

[dependencies]
# Request
## Send requests
reqwest = { version = "=0.12.20", default-features = false, features = ["cookies", "rustls-tls-native-roots", "multipart", "gzip", "brotli", "deflate", "stream"] }
reqwest_cookie_store = "=0.8.0"
cookie_store = "=0.21.1"

# TUI
## Terminal UI framework
ratatui = { version = "=0.29.0", features = ["serde"] }
## Used to parse, use key bindings files and some utilities
crokey = "=1.1.2"
## Display big texts. Only used for displaying ATAC in the homepage.
tui-big-text = "=0.7.1"
```

Output:
```markdown
| **Category / Library** | **Version** | **Reason**                                                                              |
|------------------------| ----------- | --------------------------------------------------------------------------------------- |
| **Request**            |             |                                                                                         |
| reqwest                |  0.12.20    |  Send requests                                                                          |
| **TUI**                |             |                                                                                         |
| ratatui                |  0.29.0     |  Terminal UI framework                                                                  |
| crokey                 |  1.1.2      |  Used to parse, use key bindings files and some utilities                               |
| tui-big-text           |  0.7.1      |  Display big texts. Only used for displaying ATAC in the homepage.                      |
```

## Help

```shell
Cargo depcription

        --skip-uncommented      Do not print the dependencies that are not commented
```

## Dependencies

| **Category / Library** | **Version** | **Reason**                                |
|------------------------|-------------|-------------------------------------------|
| **Serialization**      |             |                                           |
| toml_edit              |  0.22.27    |  Deserialize toml manifests               |
| **Output**             |             |                                           |
| to_markdown_table      |  0.1.5      |  Transform dependencies to markdown table |

## License

The MIT license for this project can be seen [here](https://github.com/Julien-cpsn/cargo-depcription/blob/main/LICENSE)