
FROM rustlang/rust:nightly-bullseye as builder

RUN rustup target add wasm32-unknown-unknown

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall cargo-leptos -y

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bullseye as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/target/server/release/leptos_start /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

EXPOSE 1236
CMD ["/app/leptos_start"]
