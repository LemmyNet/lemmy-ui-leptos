FROM mcr.microsoft.com/playwright:v1.44.0-jammy

WORKDIR /usr/src/app

COPY ./ ./

RUN apt update && apt -y install libssl-dev pkg-config  build-essential gcc make
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz && mv cargo-binstall $HOME/.cargo/bin
RUN cargo binstall -y cargo-leptos
RUN npm install -g pnpm
RUN pnpm install
RUN rustup target add wasm32-unknown-unknown
RUN cd end2end
RUN pnpm install
RUN cd ..

CMD INTERNAL_HOST=lemmy:8536 HTTPS=false cargo leptos end-to-end