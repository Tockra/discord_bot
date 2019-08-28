FROM alpine

RUN apk add docker openrc py-pip python-dev libffi-dev openssl-dev gcc libc-dev make rust cargo
RUN export USER=titan
RUN rc-update add docker boot

RUN pip install --upgrade pip
RUN pip install docker-compose

COPY ./dbot /home/titan/dbot/
WORKDIR /home/titan/dbot/
RUN cargo build --release
ENTRYPOINT ["/home/titan/dbot/target/release/dbot"]
