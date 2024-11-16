#!/bin/bash

rm -f programs/pong/bin/pong
cargo r -p mfs16assembler programs/pong/kb_lib_h.mfs16 programs/pong/main.mfs16 programs/pong/kb_lib.mfs16 -o programs/pong/bin/pong
