set dotenv-load := true
set positional-arguments
export SERVICE := "<TODO>"

help:
    @just --list --unsorted

build:
    cargo build
alias b := build