# Development Setup

[Back to Readme](./Readme.md)

<a name="top"></a>

- [Development Setup](#development-setup)
	- [1. Faster Compilations](#1-faster-compilations)
		- [1. 1. linux](#1-1-linux)
	- [2. Watch](#2-watch)
		- [watch install](#watch-install)
		- [watch example](#watch-example)
	- [Tarpaulin](#tarpaulin)
		- [tarpaulin website](#tarpaulin-website)
		- [Pre-Requisite](#pre-requisite)
		- [tarpaulin install](#tarpaulin-install)
		- [tarpaulin example](#tarpaulin-example)
	- [Cargo Test](#cargo-test)
	- [Cargo Expand](#cargo-expand)
	- [Linting](#linting)
	- [Code Formatting](#code-formatting)
	- [Vuln Checks](#vuln-checks)
	- [Remove unused dependencies](#remove-unused-dependencies)
	- [Postgres Setup](#postgres-setup)
	- [SQL-X](#sql-x)
	- [Run postgres via docker](#run-postgres-via-docker)

## 1. Faster Compilations

Linker config file:

### 1. 1. linux

```toml
# .cargo/config.toml

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
```

## 2. Watch

### watch install

```shell
cargo instal cargo-watch
```

### watch example

```shell
cargo watch -x check
```

```shell
cargo watch -x check -x test -x run
```

## Tarpaulin

### tarpaulin website

<https://github.com/xd009642/tarpaulin>

### Pre-Requisite

You may have to install pkg-config, try this first

```shell
sudo apt install pkg-config
```

```shell
sudo apt install libssl-dev
```

### tarpaulin install

```shell
cargo instal cargo-tarpualin
```

### tarpaulin example

```shell
cargo tarpaulin --ignore-tests
```

Generate code coverage using the following command

```shell
cargo tarpaulin --out lcov --output-dir ./coverage
```

## Cargo Test

bunyan "prettifies" the console output

```shell
sudo apt install bunyan
```

To view the tracing while running tests, use the following command:

```shell
TEST_LOG=true cargo test | bunyan
```

To omit the tracing output just run carg test by itself:

```shell
cargo test
```

## Cargo Expand

```shell
cargo install cargo-expand
```

Install the nightly build

```shell
rustup toolchain install nightly
```

now you can run the expand command using cargo

```shell
cargo +nightly expand
```

## Linting

```shell
rustup component add clippy
```

```shell
cargo clippy
```

```shell
cargo clippy -- -D warnings
```

## Code Formatting

```shell
rustup component add rustfmt
```

```shell
cargo fmt
```

```shell
cargo fmt -- --check
```

## Vuln Checks

```shell
cargo install cargo-audit
```

```shell
cargo audit
```

## Remove unused dependencies

```shell
cargo install cargo-udeps
```

```shell
cargo +nightly udeps
```

## Postgres Setup

Execute the following script to start running PostgresSQL using docker.

Install the psql client

```shell
sudo apt install postgresql-client-common postgresql-client
```

## SQL-X

This project uses SQL-X to communicate with Postgres.  Install the cli using the following command.

```shell
cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres
```

The following command exports the SQL schema and queries so that the app can work in an offline mode without having Postgres running:

'''shell
cargo sqlx prepare
'''

The file generated should be checked into source control.  This command should be ran after changing the Schema or an existing query.

## Run postgres via docker

```shell
./scripts/init_db.sh
```

to run future migrations without having to tear down the docker container you can supply "SKIP_DOCKER=true" before the script like so:

```shell
SKIP_DOCKER=true ./scripts/init_db.sh
```

[Back to top](#top)
