#!/bin/bash

cargo build --release 

samply record ./target/release/rum rum-binaries/sandmark.umz