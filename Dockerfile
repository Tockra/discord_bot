FROM alpine

RUN apk add docker rust openrc py-pip python-dev libffi-dev openssl-dev gcc libc-dev make bash cargo
RUN rc-update add docker boot

RUN pip install --upgrade pip
RUN pip install docker-compose

COPY dbot /home/titan/dbot

WORKDIR /home/titan/dbot/
RUN cargo build --release

ENTRYPOINT ["bash","/home/titan/dbot/target/release/dbot"]
