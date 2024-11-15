# TODO: Do this less badly
test backtrace="nope":
  {{ if backtrace != "nope" { "RUST_BACKTRACE=1" } else { "" } }} cargo nextest run

build:
  cargo build

crate-versions:
  cargo tree --depth 1