# Eric's Godot Resource Derive Macro

Derive macro for my specific use case:

- Working on a Godot project with Rust
- Resources written in gdscript, with Corresponding structs on the Rust side
- Too lazy to `impl Into<$TYPE> for Resource` each time

Known wacky edge cases:
- If your struct is a `GodotClass` with a base of `Resource` (i.e. if your resource is written in rust), I'm pretty sure the macro fails.
    - But also, I don't think you'd need this crate in that case?
