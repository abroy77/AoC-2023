#! /bin/bash

cargo new "$1"
cd "$1" || exit 0
rm src/main.rs
mkdir src/bin
touch src/bin/part1.rs
touch src/bin/part2.rs
mkdir data
touch data/input1.txt
