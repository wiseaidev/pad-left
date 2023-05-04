# pad-left
A simple library to left pad a string with a given character up to a certain length.

## Usage

```rust
use pad_left::left_pad;

fn main() {
    assert_eq!(left_pad("hello".to_string(), 15, '🚀'), "🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀hello".to_string());
}
```

## Run tests

```sh
cargo test
```

## Build & Open Docs

```sh
cargo doc --open
```