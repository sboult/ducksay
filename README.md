# ducksay

Make Waddles say things from your terminal.

## Install

```sh
cargo install ducksay
```

## Usage

Run with the default message:

```sh
cargo run
```

Pass a message:

```sh
cargo run -- hello from Waddles
```

Use monospace duck output:

```sh
cargo run -- --mono hello
```

Adjust wrapping:

```sh
cargo run -- --width 20 hello from Waddles
```

## License

Apache-2.0. See [LICENSE](LICENSE).
