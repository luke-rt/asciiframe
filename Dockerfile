FROM rust:1.31

WORKDIR /app

COPY . .

RUN chmod +x ci/deps-debian.sh
RUN ci/deps-debian.sh
