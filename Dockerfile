# 1: Build
FROM rust:1.58.1 as builder

# 1a: Prepare for static linking
RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev

# 1b: Create application user
RUN groupadd --gid 10001 appgroup && \
    useradd --uid 10001 --gid 10001 --shell /usr/sbin/nologin appuser
USER appuser:appgroup
WORKDIR /home/appuser/app

# 1c: Download and compile Rust dependencies using fake source code and store as a separate Docker layer
COPY --chown=appuser:appgroup crates/template-web/Cargo.toml crates/template-web/Cargo.toml
COPY --chown=appuser:appgroup .docker/main.rs crates/template-web/src/main.rs

COPY --chown=appuser:appgroup Cargo.lock Cargo.lock
COPY --chown=appuser:appgroup Cargo.toml Cargo.toml

RUN cargo build --target x86_64-unknown-linux-musl --release && rm -rf crates/

# 1d: Build the binary using the real source code
COPY --chown=appuser:appgroup crates/ crates/

RUN cargo build --target x86_64-unknown-linux-musl --release

# 2: Copy the excutable and extra files to an empty Docker image
FROM scratch

COPY --chown=root:root .docker/passwd /etc/passwd
COPY --chown=root:root .docker/group /etc/group

USER appuser:appgroup

COPY --chown=appuser:appgroup --from=builder /home/appuser/app/target/x86_64-unknown-linux-musl/release/template-web /home/appuser/template-web

CMD [ "/home/appuser/template-web" ]
