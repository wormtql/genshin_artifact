FROM rust:1.59 AS rust
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY . /mona
WORKDIR /mona/mona
RUN cargo run --release --bin gen_meta
RUN /usr/local/cargo/bin/wasm-pack build

FROM node:14-alpine AS node
RUN apk update && apk add git
WORKDIR /mona
COPY --from=rust /mona .
RUN npm install
RUN npm run build

FROM nginx AS nginx
COPY --from=node /mona/dist /usr/share/nginx/html
