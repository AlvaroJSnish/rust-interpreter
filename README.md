# rust-interpreter

Creating a simple interpreter in Rust. (Based on [Writing An Interpreter In Go](https://interpreterbook.com/))

For the Go version, see [here](https://github.com/AlvaroJSnish/go-interpreter).

## Progress

- [x] Chapter 1: Lexing
- [ ] Chapter 2: Parsing
- [ ] Chapter 3: Evaluating
- [ ] Chapter 4: Extending the Interpreter
- [ ] Going further.

## Usage

### REPL

```bash
cargo run
```

Pass any code to the REPL and it will be lexed.

Example:

```bash
let add = fn(x, y) { x + y; };
```
