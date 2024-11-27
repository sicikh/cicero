FROM node:20.16-bookworm AS build
WORKDIR /usr/src/app

COPY package*.json ./

ENV HUSKY=0

RUN --mount=type=cache,target=/usr/src/app/.npm \
  npm set cache /usr/src/app/.npm && \
  npm ci

COPY . .

RUN npm run build

FROM nginxinc/nginx-unprivileged:1.23-alpine-perl

COPY --link --from=build usr/src/app/dist/ /usr/share/nginx/html
