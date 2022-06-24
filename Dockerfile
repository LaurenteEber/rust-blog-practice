FROM ubuntu:20.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY ./ /app
RUN cargo build --release

FROM ubuntu:20.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y
WORKDIR /app

# COPY --from=0 /app/.env /app
COPY --from=0 /app/target/release/blog-project /app
COPY /templates/ /app/templates

CMD ./blog-project

# docker run -d --name blog-project -e "PORT=8765" -e "DEBUG=0" -p 8007:8765 web:latest