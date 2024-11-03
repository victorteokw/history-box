History Box
===========

A history box that keeps history references

## Installation

Add these lines to `Cargo.toml` under the `[dependencies]` section:

```toml
history-box = "0.1"
```

## Usage

```rust
let history_box = HistoryBox::new();
history_box.get(); // None
history_box.set(1);
history_box.get(); // Some(&1))
history_box.set(2);
history_box.get(); // Some(&2))
history_box.set(3);
history_box.get(); // Some(&3))
```

## License

MIT License