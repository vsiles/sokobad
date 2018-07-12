# sokobad
Very ugly clone of sokoban (inspired by Hinder) to test writing some rust code

Depends on SDL2 for rust.

# How to play:
- Use the arrow keys to move the character (red block)
- Move the crates (brown blocks) on the goal cells (yellow block). They should turn 'light green'.
- Once all the goal cells are green, the exit (black block) will open (white block).
- Proceed to the exit

HOWTO:

```
$ cargo build --release
$ cargo run < data/maps/map0
```
