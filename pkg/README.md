# Minesweeper

[here](https://youtu.be/0ywizYLPV00?list=PLcK845XHyaAGoNNS9I6C9DSzv8gBxEPGK&t=4035)

Build for web

```rust
wasm-pack build --target web
```

Run static server
```sfz```

- insert mine
- flag field
- open field
- status game
- first opening is safe
- win if there is : **mines == all_map - opened_fields + flag_fields**
