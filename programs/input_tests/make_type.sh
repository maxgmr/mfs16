#!/bin/bash

rm -f programs/type
cargo r -p mfs16assembler programs/libs/kb_lib_h.mfs16 programs/input_tests/type.mfs16 programs/libs/kb_lib.mfs16 programs/fonts/5x7basic.mfs16 -o programs/type
