#!/bin/sh
sudo apt-get update
sudo apt-get install -y cmake g++ wget unzip clang libclang-dev libprotobuf-dev protobuf-compiler

wget -O opencv.zip https://github.com/opencv/opencv/archive/4.x.zip && unzip opencv.zip

mkdir -p build && cd build
cmake ../opencv-4.x && cmake --build .
