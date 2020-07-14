FROM rustlang/rust:nightly-stretch

WORKDIR /usr/src/periodicaidan_xyz
COPY . .

RUN cargo install --path .
CMD ["periodicaidan_xyz"]