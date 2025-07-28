# Build Rust artifacts and WASM
FROM rust:latest AS rust-builder
RUN rustup toolchain install nightly-2024-10-10
RUN rustup default nightly-2024-10-10
RUN apt-get update && apt-get install -y \
    git \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install wasm-pack
WORKDIR /mona
COPY . .

# Build mona_generate and generate metadata
WORKDIR /mona/mona_generate
RUN cargo build --release --bin gen_meta && \
    ./target/release/gen_meta --dir ../src/assets --i18n-dir ../src/i18n/generated

# Build mona_wasm
WORKDIR /mona/mona_wasm
RUN wasm-pack build --release

# Build mona_core first as dependency (production build skips tests)
# For test builds, use: 
# docker build --target rust-builder -t genshin-artifact-test . && docker run -it genshin-artifact-test bash -c "cd /mona/mona_core && cargo test"
WORKDIR /mona/mona_core
RUN mv src/bin/test.rs src/bin/test.rs.bak && \
    mv src/bin/test_damage.rs src/bin/test_damage.rs.bak && \
    cargo build --release && \
    mv src/bin/test.rs.bak src/bin/test.rs && \
    mv src/bin/test_damage.rs.bak src/bin/test_damage.rs

# Build frontend
FROM node:20-alpine AS node-builder
RUN apk add --no-cache git
WORKDIR /mona
COPY --from=rust-builder /mona .
RUN npm install
RUN npm run build

# Final production image
FROM nginx:alpine
COPY --from=node-builder /mona/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
