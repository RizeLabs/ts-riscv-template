#!/bin/bash

# deleting existing content
rm -rf ../wasm/* 

cd package

# compile ts to wasm
npm run compile