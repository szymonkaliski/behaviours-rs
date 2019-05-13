#!/usr/bin/env bash

cargo watch -i "pkg/*" -i "test/*" -i "target/*" -s "wasm-pack build"
