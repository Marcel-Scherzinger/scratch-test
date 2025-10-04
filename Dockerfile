FROM rust:1.90 AS build
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /usr/src/service
# Dummy packages
RUN mkdir -p crates/model/src crates/interpreter/src crates/testreports/src crates/testdata/src crates/scratch-yew/src src && echo "fn main() {}" > src/main.rs && touch crates/interpreter/src/lib.rs && touch crates/model/src/lib.rs && touch crates/testreports/src/lib.rs && touch crates/testdata/src/lib.rs
COPY Cargo.* .
COPY crates/model/Cargo.toml crates/model/
COPY crates/interpreter/Cargo.toml crates/interpreter/
COPY crates/testreports/Cargo.toml crates/testreports/
COPY crates/testdata/Cargo.toml crates/testdata/
COPY crates/scratch-yew/Cargo.toml crates/scratch-yew/
COPY crates/scratch-yew/.cargo crates/scratch-yew/.cargo/

# Only build dependencies
WORKDIR /usr/src/service/crates/scratch-yew
RUN echo "" > index.scss && echo "<html><head><link data-trunk rel=\"rust\" /><link data-trunk rel=\"sass\" href=\"index.scss\" /></head><body></body></html>" > index.html && echo 'use yew::prelude::*;\n#[function_component(App)]\nfn app() -> Html\n    { return html!("deps"); }\n fn main() { yew::Renderer::<App>::new().render(); }' > src/main.rs && cat src/main.rs
RUN trunk build
RUN trunk build --release
WORKDIR /usr/src/service
RUN rm -rf crates

# Now add own source code and build release
COPY crates/ crates/
RUN find crates -type f -exec touch -a -m {} + && cd crates/scratch-yew && trunk build --release

EXPOSE 8080

FROM nginx:1.29.0 AS run
WORKDIR /page
RUN rm /etc/nginx/conf.d/ -r && mkdir /etc/nginx/conf.d
COPY --from=build /usr/src/service/crates/scratch-yew/dist/ /page/dist/
COPY crates/scratch-yew/nginx-frontend.conf /etc/nginx/conf.d/

