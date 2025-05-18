# shears-mre

Minimal reproducible example for https://github.com/Boshen/cargo-shear/issues/184

## How to reproduce

Before, run:

```bash
cargo +nightly  test --features nightly
```

The tests will pass.

Then run:

```bash
cargo shear --fix
```

It will generate a diff removing hashbrown:
```
➜  shears-mre git:(main) ✗ cargo shear --fix                      
Analyzing /home/user/Projects/shears-mre

shears-mre -- Cargo.toml:
  hashbrown

Fixed 1 dependencies!
No unused dependencies!
```

Now run again:
```bash
cargo +nightly  test --features nightly
```

It will fail with the following error:
```
error: feature `nightly` includes `hashbrown/nightly`, but `hashbrown` is not a dependency
 --> Cargo.toml:8:11
  |
8 | nightly = ["hashbrown/nightly"]
  |           ^^^^^^^^^^^^^^^^^^^^^
  |
error: failed to parse manifest at `/home/user/Projects/shears-mre/Cargo.toml`
```
