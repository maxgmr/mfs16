#!/bin/bash
rm -f -- $2
cargo r -p mfs16assembler $1 -o $2
cargo r -p mfs16desktop $2
