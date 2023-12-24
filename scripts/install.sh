#!/bin/bash

echo "Build RISC0 prpject..."
# build your rust project
cargo build

# getting to the package path
cd package

echo "Installing node packahes..."
# installing packages
npm install