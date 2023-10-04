# Build stage for the backend
FROM rust:1.72-buster as build

WORKDIR /app

# Accept building
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

# Copy the source code
COPY . .

RUN cargo build --release



# production build stage

FROM debian:buster-slim

WORKDIR /usr/local/building

COPY --from=builder /app/target/release/rust-crud-api .

CMD [ "./rust-crud-api" ]
