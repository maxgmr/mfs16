#!/bin/bash

rm -f programs/bouncing_logo
cargo r -p mfs16assembler programs/graphic_tests/logo.mfs16 programs/graphic_tests/bouncing_logo.mfs16 -o programs/bouncing_logo
