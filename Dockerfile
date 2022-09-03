FROM rust:latest

# install tesseract dependencies
RUN apt-get update && \
    apt-get install -y libleptonica-dev libtesseract-dev clang tesseract-ocr-eng

WORKDIR /usr/src/doom_alerts

COPY . .

RUN cargo install --path .

CMD ["doom_alerts", "&"]
