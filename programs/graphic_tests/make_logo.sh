#!/bin/bash

rm -f programs/logo
cargo r -p mfs16assembler programs/graphic_tests/logo.mfs16 -o programs/logo
