FROM rust:1.32-slim

RUN cargo install cargo-watch
RUN rustup component add rustfmt
RUN rustup component add clippy

RUN apt update
RUN apt-get -y install locales
RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
    locale-gen
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8
RUN apt install -y libncurses5-dev
RUN apt install -y libncursesw5-dev

WORKDIR /data
VOLUME /data
COPY . .
