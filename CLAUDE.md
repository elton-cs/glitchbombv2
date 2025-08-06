# Claude Project Memory

## Build Commands

For this Bevy project, use:
- `bevy build web` - For web compilation and checking
- NOT `cargo run` or `cargo build` - These are too slow for development iteration

## UI/Interface Guidelines

**ALWAYS use flexbox layout for UI and interface elements:**
- Use `flex_direction`, `justify_content`, `align_items` for positioning
- Use `margin`, `row_gap`, `column_gap` for spacing
- NEVER use absolute positioning (`PositionType::Absolute`) unless absolutely necessary
- Prefer `Val::Percent()` for responsive layouts, `Val::Px()` for fixed sizes
- Use flexbox containers to organize UI hierarchies properly

## Project Structure

- `interface/` - UI layer plugin for the game
- Main game uses Bevy engine with custom plugins