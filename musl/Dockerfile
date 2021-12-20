FROM alpine:3.15.0

RUN apk update
RUN apk add curl
RUN apk add zip
RUN apk add glib
RUN apk add glib-dev
RUN apk add pango
RUN apk add pango-dev
RUN apk add cairo
RUN apk add cairo-dev
RUN apk add atk
RUN apk add atk-dev
RUN apk add gtk+3.0
RUN apk add gtk+3.0-dev
RUN apk add bash
RUN apk add gcc
RUN apk add musl-dev


RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile default --default-toolchain nightly
ENV PATH=$PATH:/root/.cargo/bin
WORKDIR /project
