# gdrust

example addon integrating [Rust-lang](https://rust-lang.org) into [Godot Engine](https://godotengine.org) using [godot-rust](https://godot-rust.github.io)

## see also

[godot-rust book](https://godot-rust.github.io/book) \
[Godot Engine documentation](https://docs.godotengine.org/en/stable) \
[Learn Rust](https://www.rust-lang.org/learn) \
</x-nul>

## features

- hot reload (tested in Godot 4.4)

## implementation status

- [x] singletons
- [x] refcounted
- [x] nodes
- [x] properties (`var` and `export`)
- [x] virtual functions
- [x] concrete functions
- [x] lifecycle functions (e.g. ready, process)
- [x] associated functions (e.g. constructors)
- [ ] signals ([:warning: unstable](https://godot-rust.github.io/book/register/signals.html))

## todo

- containerization and GitHub workflows (formatting, builds, exports, tests)
