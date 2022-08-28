FROM rust:1.63.0-alpine as deps

RUN apk add --no-cache musl-dev
RUN apk add nodejs npm

WORKDIR /usr/src/rutter

# install dependencies in seperate layer for faster docker builds
COPY Cargo.toml .
# create fake project
RUN mkdir -p src && echo "fn main() {}" > ./src/main.rs
# install deps
RUN cargo build

# Build the frontend
COPY /frontend ./frontend
WORKDIR /usr/src/rutter/frontend
RUN npm i
RUN npm run build

WORKDIR /usr/src/rutter/

# CMD [ "sh" ]

# Build actual image
FROM rust:1.63.0-alpine as app
WORKDIR /usr/src/rutter

# COPY --from=deps /rutter/Cargo.toml ./rutter
COPY --from=deps /usr/local/cargo /usr/local/cargo
COPY --from=deps /usr/src/rutter/static /usr/src/rutter/static

RUN apk add --no-cache musl-dev

COPY . .

# CMD [ "tail", "-f", "/dev/null" ]
CMD ["cargo", "run", "--release"]
