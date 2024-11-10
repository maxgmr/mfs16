#!/bin/bash

rm -f programs/type
cargo r -p mfs16assembler programs/kb_lib.mfs16 programs/input_tests/type.mfs16 programs/fonts/5x7basic.mfs16 -o programs/type
