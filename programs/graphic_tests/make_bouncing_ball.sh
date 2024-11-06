#!/bin/bash

rm -f programs/bouncing_ball
cargo r -p mfs16assembler programs/graphic_tests/bouncing_ball.mfs16 -o programs/bouncing_ball
