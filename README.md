To see a good example of how it works run this command;
```bash
B=43 cargo run -- -f all.env -c 44
```

You'll get;
```rust
Options { file: Some("all.env"), a: 42, b: 43, c: 44 }
```

FYI, `all.env`;
```bash
A = 42
B = 4
C = 2
```

Thus in the previous example you've seen the env file being used for setting `A`,
the current env being used for `B` and the cli being used for `C`.
