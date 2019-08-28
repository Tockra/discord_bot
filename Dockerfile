FROM rust:latest as cargo-build
COPY ./dbot ./dbot/
WORKDIR /dbot/

RUN cargo build --release
RUN cargo install --path .

FROM alpine

RUN apk add docker openrc py-pip python-dev libffi-dev openssl-dev gcc libc-dev make
RUN export USER=titan
RUN rc-update add docker boot

RUN pip install --upgrade pip
RUN pip install docker-compose

COPY --from=cargo-build /usr/local/cargo/bin/dbot /usr/local/bin/

CMD ["dbot"]
