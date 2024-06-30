FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /mogcord

COPY . .

#Release
#RUN cargo build --release --target x86_64-unknown-linux-musl
#Debug
RUN cargo build --target x86_64-unknown-linux-musl

FROM scratch
#Release
#COPY --from=builder /mogcord/target/x86_64-unknown-linux-musl/release/mogcord /mogcord
#Debug
COPY --from=builder /mogcord/target/x86_64-unknown-linux-musl/debug/mogcord /mogcord
ENTRYPOINT ["/mogcord"]
EXPOSE 3000