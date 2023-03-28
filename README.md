# glip

not much to say here, this is a small library to let you set and get text from the global clipboard.  basically, it's frustrating for the clipboard to disappear as soon as your program finishes running, so this is a wrapper on the cli commands for global clipboards in linx, mac, and windows.

### usage
```rust
use glcp::GlobalClip;
use anyhow::Result;

fn main() -> Result<() {
  let value = "set it here";
  GlobalClip::set(value);?
  let from_clipboard = GlobalClip::get()?;
  assert_eq!(value, from_clipboard)
}
```
