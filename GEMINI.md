# rSnake Project Mandates

This document serves as the foundational guide for any AI interactions within the `rSnake` workspace. All instructions here take absolute precedence over general defaults.

## Project Overview
`rSnake` is a high-performance, flicker-free terminal Snake game built in Rust using `crossterm`. It features a state-driven architecture, targeted rendering, and a persistent high-score system.

## Architectural Principles
- **Separation of Concerns**:
    - `src/game.rs`: Pure game logic and state management.
    - `src/render.rs`: Terminal-specific rendering logic (using `UpdateResult` for differential updates).
    - `src/high_scores.rs`: File-based persistence.
    - `src/main.rs`: Orchestration, input polling, and mode switching.
- **Differential Rendering**: NEVER use `Clear(ClearType::All)` during active gameplay. Always use `UpdateResult` to perform targeted updates (drawing only the new head and clearing only the old tail) to maintain a flicker-free experience.
- **Type Safety**: Ensure all coordinates use `u16` to match `crossterm` expectations.

## Engineering Standards
- **Testing**: Every logic change in `game.rs` must be accompanied by a unit test. Run `cargo test` to verify.
- **Dependencies**: The project strictly uses `crossterm` for terminal manipulation and `rand` for game mechanics. Avoid adding heavy dependencies unless absolutely necessary.
- **Performance**: The game loop runs at a fixed tick rate (default 100ms). Any added logic must be non-blocking.

## Workspace Commands
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: `cargo test`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`

## Security & Integrity
- Do NOT commit `high_score.txt` or the `target/` directory.
- Maintain `.gitignore` to exclude local state and build artifacts.
