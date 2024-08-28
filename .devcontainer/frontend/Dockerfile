FROM oven/bun:1

# Config Bun
ENV PATH="~/.bun/bin:${PATH}"
RUN ln -s /usr/local/bin/bun /usr/local/bin/node

# Update packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && apt-get install -y git

COPY ../.env /.env