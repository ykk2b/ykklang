#!/bin/bash

mdbook build -d docs
cargo build --release