# stubthat-core

Minimal StubTHAT enforcement core.

**Language Pie sector:** Rust (Native/performance engine)  
**Maturity:** stubbed_out  
**Status:** candidate, not a proven runtime

## What it does

Receives a Bootstrap English ticket on the command line and returns a source-bound outpoint. Empty tickets hold. No arbitrary command execution.

```bash
cargo run -- canvas
# outpoint:ticket=canvas status=stubbed
```

## Tests

```bash
cargo test
```

## Doctrine

- Language Pie: https://github.com/marvelousempire/philosophy-language-pie
- StubTHAT in the pie: see philosophy-language-pie `docs/STUBTHAT-IN-THE-PIE.md`
- Automata V5 names Rust as primary machinery; this is one candidate surface

This skeleton proves the Rust wedge can exist as a small, inspectable binary. It does not claim acceptance or dispatch authority.
