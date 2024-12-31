# TODO: Do this less badly
test filter="" backtrace="nope":
  {{ if backtrace != "nope" { "RUST_BACKTRACE=1" } else { "" } }} cargo nextest run {{ if filter != "" { filter } else { "" } }}

build:
  cargo build

crate-versions:
  cargo tree --depth 1

