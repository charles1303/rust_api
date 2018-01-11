FROM adreeve/rust-nightly:2015-07-02

RUN aptitude install build-essential curl git clang checkinstall

RUN curl https://sh.rustup.rs -sSf | sh

RUN apt-get install libmysqlclient-dev

RUN apt-get install libc-dev

RUN cargo install diesel_cli

RUN cargo install diesel_cli --no-default-features --features mysql

ADD . /source
WORKDIR /source

EXPOSE 9000

RUN diesel setup
RUN diesel migration run
RUN rustc -V
RUN cargo -V
RUN cargo build

CMD cargo run