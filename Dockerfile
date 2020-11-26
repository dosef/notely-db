FROM rust:1.46 as builder
WORKDIR /notely-db
COPY . .
RUN rustup update nightly;
RUN rustup default nightly;
RUN cargo build

FROM centos:8
WORKDIR /notely-db
COPY --from=builder notely-db/target/debug/notely .
RUN yum install -y postgresql-contrib
CMD ["./notely"]
