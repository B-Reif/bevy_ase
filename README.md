Basic tooling to use Aseprite animations with Bevy. Forked from bevy_proto_aseprite.

# Example

```
cargo run --example simple
```

# TODOs

- Improve error handling.

- Atlas creation fails if there are too many / too big sprites.

- Way to designate sprites as tiles. Right now, you sometimes get small lines
  between tiles due to rounding errors (mainly when resizing window). This can
  be prevented by adding a 1px edge around each tile. But we only want to do
  this for specific tiles.

- Hot reloading. This requires dynamic atlas reconstruction.
