FROM rust:1-slim-bullseye AS rust-builder

RUN apt update && apt install -y pkg-config libasound2-dev

WORKDIR /sound

COPY ./sound/Cargo.toml ./sound/Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

# This first stage was used to take advantage of Docker cache to cache rust
# dependancies

COPY ./sound/src ./src

RUN cargo build --release

FROM python:3-slim-bullseye AS runner

RUN apt update && apt install -y curl

WORKDIR /usr/src/sound

COPY --from=rust-builder /sound/target/release/sound ./

COPY ./requirements.txt ./main.py ./

RUN pip install -r requirements.txt

EXPOSE 5000
ENTRYPOINT ["python"]
CMD ["main.py"]


