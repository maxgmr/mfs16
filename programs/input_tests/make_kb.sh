#!/bin/bash

rm -f programs/kb
cargo r -p mfs16assembler programs/libs/kb_lib_h.mfs16 programs/input_tests/kb.mfs16 programs/libs/kb_lib.mfs16 -o programs/kb
