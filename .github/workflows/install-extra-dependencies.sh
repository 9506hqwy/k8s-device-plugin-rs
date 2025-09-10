#!/bin/bash
set -euo pipefail

# install protoc
sudo apt update -y
sudo apt install -y protobuf-compiler
