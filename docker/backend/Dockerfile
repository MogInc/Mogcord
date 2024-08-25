ARG API_PORT=3000
ARG BUILD_TYPE=debug

FROM node:14-alpine as tailwind-builder
WORKDIR /mogcord

COPY tailwind.config.js ./
COPY package*.json ./
COPY templates/ templates/
# can do this in the last step but found it easier to do here
COPY public/static/js/htmx.js static/js/

RUN npm install && npx tailwindcss -i templates/input.css -o static/styling/main.css

FROM messense/rust-musl-cross:x86_64-musl as rust-builder
WORKDIR /mogcord

COPY . .

RUN if [ "$BUILD_TYPE" = "release" ]; then cargo build --release --target $CARGO_BUILD_TARGET; else cargo build --target $CARGO_BUILD_TARGET; fi

FROM scratch

ARG API_PORT
ARG BUILD_TYPE
ENV API_PORT=$API_PORT
ENV BUILD_TYPE=$BUILD_TYPE

COPY --from=rust-builder /mogcord/target/x86_64-unknown-linux-musl/$BUILD_TYPE/mogcord /mogcord
COPY --from=tailwind-builder /mogcord/static/ static/

ENTRYPOINT ["/mogcord"]
EXPOSE ${API_PORT}