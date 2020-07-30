FROM alpine

RUN apk --update --no-cache add docker openrc py-pip python3-dev libffi-dev openssl-dev gcc libc-dev make rust cargo
RUN rc-update add docker boot

RUN pip install --upgrade pip
RUN pip install docker-compose
COPY ./dbot /home/titan/dbot/
WORKDIR /home/titan/dbot/
RUN cargo build --release

CMD ["/home/titan/dbot/target/release/dbot"]
