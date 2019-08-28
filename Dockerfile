FROM rust as builder
COPY dbot ./
RUN cargo build --release 

FROM alpine

RUN apk add docker openrc py-pip python-dev libffi-dev openssl-dev gcc libc-dev make
RUN rc-update add docker boot

RUN pip install --upgrade pip
RUN pip install docker-compose

COPY --from=builder ./target/release/dbot /home/titan/dbot/
ENTRYPOINT ["/home/titan/dbot/dbot"]
