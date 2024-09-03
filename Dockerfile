FROM oven/bun:1 as frontend

ENV PATH="~/.bun/bin:${PATH}"
RUN ln -s /usr/local/bin/bun /usr/local/bin/node

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && apt-get install -y git

WORKDIR /usr/src/

COPY ./frontend .

RUN bun install && bun build

FROM rustlang/rust:nightly-slim as backend

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=frontend /usr/src/dist /usr/app/frontend/dist
COPY --from=frontend /usr/src/dist/index.html /usr/app/frontend/dist/index.html
COPY --from=backend /usr/src/config /usr/app/config
COPY --from=backend /usr/src/target/release/cicero-cli /usr/app/cicero-cli

ENTRYPOINT ["/usr/app/cicero-cli"]