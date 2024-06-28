#!/bin/bash

cargo build --release
scp -r target/release/online-test let@192.168.89.89:code/builds/
