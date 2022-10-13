# terminal menu

[terminal-menu-rs](https://gitlab.com/xamn/terminal-menu-rs/-/tree/master/)

# Note

Command to clear screen

## Linux or macOS terminal

```rust
std::process::Command::new("clear").status().unwrap();
```

## Windows

```rust
std::process::Command::new("cls").status().unwrap();
```

This basically sends the `clear` command to terminal.
