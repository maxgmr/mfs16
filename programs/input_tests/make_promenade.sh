#!/bin/bash

rm -f programs/promenade
cargo r -p mfs16assembler programs/libs/kb_lib_h.mfs16 programs/input_tests/promenade.mfs16 programs/libs/kb_lib.mfs16 -o programs/promenade
