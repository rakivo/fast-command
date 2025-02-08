# [fast-command](https://github.com/rakivo/fast-command)

## Simple implementation of `Command` to avoid fork + exec overhead.
> Used in [`rush`](https://github.com/rakivo/rush) build system.

# Simple example
```rust
use fast_command::{Output, Command};

fn main() -> std::io::Result::<()> {
    let Output { status, stdout, stderr } = Command::new("echo hello").execute()?;

    println!("status = {status}");
    println!("stdout = {stdout:?}");
    println!("stderr = {stderr:?}");

    Ok(())
}
```
