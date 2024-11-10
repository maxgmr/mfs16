#!/bin/bash

rm -f programs/kb
cargo r -p mfs16assembler programs/input_tests/kb.mfs16 -o programs/kb
