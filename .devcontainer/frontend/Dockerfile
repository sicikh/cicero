FROM node:20.16-bookworm

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && apt-get install -y git

COPY ../.env /.env