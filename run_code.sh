#! /bin/sh
clear
rustc --crate-name foo $1 --test
./foo
rm ./foo