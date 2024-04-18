# oargs

Argument Parser

## Install

```bash
cargo add oargs
```

or

```toml
[dependencies]
oargs = "0.1.0"
```

## Example

```rust
use oargs::{arg, Args};

fn main() {
    let args = vec![
        arg!({
            name: "about_arg",
            short: "a",
            long: "about"
        }),
        arg!({
            name: "version_arg",
            short: "v",
            long: "version"
        }),
    ];

    let args_obj = Args::new(args, std::env::args().collect());

    if args_obj.contains("help_arg") {
        println!("Help");
    }

    if args_obj.contains("version_arg") {
        println!("Version {}", std::env!("CARGO_PKG_VERSION"));
    }
}
```
