FROM paritytech/ci-linux:production

ADD . /levia
WORKDIR /levia
RUN cargo build --release

EXPOSE 9944
ENTRYPOINT ["target/release/node-template"]
CMD ["--dev", "--ws-external", "--rpc-external"]