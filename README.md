# Really simple calculator in Rust

This is a really simple calculator that can parse and compute mathematical expressions.

## Rust 

This project is written in Rust and uses the `tracing` crate for logging.

The parser is a top-down operator precedence parser. It supports the following operators:

- `+`
- `-`
- `*`
- `/`

And the following functions:

- `sin`
- `cos`
- `tan`
- `exp`
- `log`
- `log2`
- `log10`
- `sqrt`

It also supports parentheses and nested expressions.

And supports unary operators `+` and `-`.

This has been written to be a simple educational tool to understand how to parse mathematical expressions in Rust without any external dependencies.

## Usage

```
cargo run -- --help
```

## Examples

```
cargo run -- "1 + 2 * sin(1.9)"
```

![Screenshot](screenshot.png)

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.
