#!/bin/bash

rm -f programs/hello_world
cargo r -p mfs16assembler programs/graphic_tests/hello_world.mfs16 programs/graphic_tests/test_pattern.mfs16 programs/fonts/5x7basic.mfs16 -o programs/hello_world
