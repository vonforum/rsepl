# RsEPL

RsEPL is a REPL not tied to a compiler version - instead of using the compiler internals to
compile and run your code, we just write it to a file and run the compiler on that. This
makes it much slower but future-proof.

## Prerequisites, installation and usage

RsEPL requires:

* cargo to be installed and available in PATH
* user data directory to be writable (AppData\Roaming in Windows, $HOME/.local/share or $XDG_DATA_Home on Linux etc)

Install with cargo:

```sh
cargo install rsepl
```

Run as `rspl`

### Commands

RsEPL has a few commands, all beginning with `:`:

* `:exit` - Exits the program. You can also exit with CTRL-D or CTRL-C
* `:h` or `:help` - List commands
* `:buffer` - Print all code you've entered so far (which will all be compiled)
* `:clear` - Clear the buffer
* `:pop` - Remove the last successful line from the buffer (lines that error are not added anyway)

## Caveats

This runs quite slowly. Everything you write is appended to a list, which is all written to a
rust source file, which is then compiled and run with cargo. This means that every time you run a line
all previous lines are also recompiled. Currently there's also no removal of no longer useful lines
(one-offs like `2+2` etc).

## Future

Features I wish to add:

* remove no longer useful lines
* add handling of multiline code - blocks etc
* `extern crate`s - pull from crates.io and use in REPL

## Name

Can be interpreted as anything from: *Read - (slowly) Evaluate - Print - Loop* to *Rust, Evaluate, Print, Loop*

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
