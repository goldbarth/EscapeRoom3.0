# Escape Room 3.0 - Rust

An early Rust console-menu prototype, started after building the escape-room idea in [C#](https://github.com/goldbarth/EscapeRoom) and [C++](https://github.com/goldbarth/EscapeRoom2.0).
This was my first experiment with Rust.

**The current source does not implement a playable escape-room game.**
It contains a title screen and a selectable menu, with an unfinished application-state structure.

## Implemented so far

- Character-art title screen and English menu labels.
- Menu options for `Start Game`, `Tutorial`, and `Exit`.
- Input handling for Up, Down, and Enter using Crossterm.
- A message showing which option was selected.
- Separate modules for application flow, menus, and console helpers.

## Current behavior and limitations

Selecting a menu entry prints its name, then returns to the main menu.
The selection is not connected to an application-state change, so `Start Game` does not launch gameplay and `Exit` does not close the application.
Use Ctrl+C to stop the process.

Room creation, player movement, key collection, exit unlocking, and a win condition have not been implemented.
Tutorial and outro methods contain placeholder text and are not reached through the menu.
The console-clear helper constructs a Crossterm command without executing it, and terminal raw mode is not enabled.
Keyboard behavior and screen clearing therefore still need work.

## Build and run

The Cargo project lives in `rust_escape_room_2/`, despite the repository being named `EscapeRoom3.0`.
Its package name is also still `rust_escape_room_2`, with version `0.1.0` and Rust edition 2021.

Install Rust and Cargo with support for edition 2021, then use:

```sh
git clone https://github.com/goldbarth/EscapeRoom3.0.git
cd EscapeRoom3.0/rust_escape_room_2
cargo run --locked
```

To build without starting the prototype:

```sh
cargo build --locked
```

Run it in an interactive terminal.
The manifest declares `crossterm = "0.27.0"`, `crossterm_cursor = "0.4.0"`, and `time = "0.3.30"`.
The committed lockfile records the dependency versions; no Rust toolchain version is pinned in the repository.

## Code structure

| File | Responsibility |
| --- | --- |
| `rust_escape_room_2/src/main.rs` | Module declarations, cursor hiding, and application startup |
| `rust_escape_room_2/src/application.rs` | Application loop and unfinished state dispatch |
| `rust_escape_room_2/src/menu.rs` | Title art, menu drawing, and keyboard-event handling |
| `rust_escape_room_2/src/console.rs` | Cursor-position and console-clear helpers |
| `rust_escape_room_2/Cargo.toml` | Package metadata and dependencies |
| `rust_escape_room_2/Cargo.lock` | Locked dependency versions |

## Related projects

| Repository | Language | Scope |
| --- | --- | --- |
| [EscapeRoom](https://github.com/goldbarth/EscapeRoom) | C# | Original console game with German menus |
| [EscapeRoom2.0](https://github.com/goldbarth/EscapeRoom2.0) | C++ | Windows console reimplementation with English menus |
| [EscapeRoom3.0](https://github.com/goldbarth/EscapeRoom3.0) | Rust | Early menu prototype; gameplay is not implemented |
