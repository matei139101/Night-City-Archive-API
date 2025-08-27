FROM rust:1.85.1 AS builder

WORKDIR /usr/src/Night-City-Archive-API
COPY . .

# Define a build argument for the profile
ARG BUILD_PROFILE=debug

# Conditionally run cargo build based on the profile
RUN if [ "$BUILD_PROFILE" = "release" ]; then \
        cargo build --release; \
    elif [ "$BUILD_PROFILE" = "debug" ]; then \
        cargo build; \
    else \
        echo "Error: Invalid BUILD_PROFILE '$BUILD_PROFILE'. Must be 'release' or 'debug'."; \
        exit 1; \
    fi

FROM rust:1.85.1-slim
WORKDIR /usr/src/Night-City-Archive-API

# Define a build argument for the profile
ARG BUILD_PROFILE=debug

COPY --from=builder /usr/src/Night-City-Archive-API/target/${BUILD_PROFILE}/NCArchiveAPI .
EXPOSE 3000
CMD ["./NCArchiveAPI"]