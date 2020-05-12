#!/bin/sh
# had to add printlns to ensure to optimization out but i don't want it clocking the pipes
cargo run -q --release --bin $1 -- $2 >/dev/null
