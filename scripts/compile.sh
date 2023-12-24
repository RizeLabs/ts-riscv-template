#!/bin/bash

# deleting existing content
echo "Deleting existing content..."
rm -rf ../wasm/* 

cd package

echo "Compiling to wasm..."
# compile ts to wasm
npm run compile