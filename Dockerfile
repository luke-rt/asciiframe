FROM rust:1.31

WORKDIR /usr/src/asciiframe

RUN apt-get update
RUN apt-get install -y clang libopencv-dev

COPY . .
