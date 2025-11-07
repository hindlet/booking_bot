# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

# ARG RUST_VERSION=1.85.0
# ARG APP_NAME=booking_bot

# ################################################################################
# # Create a stage for building the application.

# FROM rust:${RUST_VERSION}-alpine AS build
# ARG APP_NAME
# WORKDIR /app



# # Install host build dependencies.
# RUN apk add --no-cache clang lld musl-dev git

# # Build the application.
# # Leverage a cache mount to /usr/local/cargo/registry/
# # for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# # for git repository dependencies, and a cache mount to /app/target/ for
# # compiled dependencies which will speed up subsequent builds.
# # Leverage a bind mount to the src directory to avoid having to copy the
# # source code into the container. Once built, copy the executable to an
# # output directory before the cache mounted /app/target is unmounted.
# RUN --mount=type=bind,source=src,target=src \
#     --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
#     --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
#     --mount=type=cache,target=/app/target/ \
#     --mount=type=cache,target=/usr/local/cargo/git/db \
#     --mount=type=cache,target=/usr/local/cargo/registry/ \
# cargo build --locked --release && \
# cp ./target/release/$APP_NAME /bin/server

# ################################################################################
# # Create a new stage for running the application that contains the minimal
# # runtime dependencies for the application. This often uses a different base
# # image from the build stage where the necessary files are copied from the build
# # stage.
# #
# # The example below uses the alpine image as the foundation for running the app.
# # By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# # reproducibility is important, consider using a digest
# # (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
# FROM alpine:3.18 AS final

# # Create a non-privileged user that the app will run under.
# # See https://docs.docker.com/go/dockerfile-user-best-practices/
# ARG UID=10001
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser
# USER appuser

# # Copy the executable from the "build" stage.
# COPY --from=build /bin/server /bin/

# COPY assets/bookings.db /bin/assets/bookings.db
# COPY assets/token.txt /bin/assets/token.txt


# # Expose the port that the application listens on.
# EXPOSE 3600

# # What the container should run when it is started.
# CMD ["/bin/server"]

# Use the official Rust image as a base
FROM rust:1.85

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml file into the working directory
COPY Cargo.toml .
COPY src/* ./src/
COPY src/db_helper/* ./src/db_helper/
# COPY assets/booking_bot .
COPY assets/bookings.db ./assets/bookings.db
COPY assets/token.txt ./assets/token.txt

# Build the Rust application
RUN cargo build --release

# Copy the built application into the working directory
# COPY target/release/booking_bot .

# Expose the port that the application will listen on
EXPOSE 8080

# Run the command to start the application when the container is launched
CMD ["./target/release/booking_bot"]