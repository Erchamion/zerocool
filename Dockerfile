FROM lukemathwalker/cargo-chef:latest-rust-1.68.2 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zerocool

FROM debian:bullseye-slim as runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zerocool zerocool
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zerocool"]
