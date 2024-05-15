FROM rust:slim-bookworm AS base
RUN apt update && apt -y install libssl-dev pkg-config build-essential gcc make wget
RUN wget -O- https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xvz -C /usr/local/cargo/bin

FROM base AS non-leptos-rust
COPY --from=base /usr/local/cargo/bin/ /usr/local/cargo/bin/
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

FROM non-leptos-rust AS leptos-rust
COPY --from=base /usr/local/cargo/bin/ /usr/local/cargo/bin/
COPY --from=non-leptos-rust . .
COPY style style
COPY public public
COPY locales locales
COPY .leptosfmt.toml .leptosfmt.toml
COPY package.json package.json
COPY pnpm-lock.yaml pnpm-lock.yaml
RUN rustup target add wasm32-unknown-unknown
RUN cargo-binstall -y cargo-leptos
RUN wget -O- https://deb.nodesource.com/setup_20.x | bash
RUN apt-get install -y nodejs
RUN npm install -g pnpm
RUN pnpm install

FROM leptos-rust AS playwright
RUN pnpx playwright@1.44.0 install --with-deps