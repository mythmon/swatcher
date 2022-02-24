# This Dockerfile uses multi-stage builds to produce very small deployed images
# and optimize usage of layer caching. Docker 17.05 or higher required for
# multi-stage builds.

# Updating this argument will clear the cache of the package installations
# below. This will cause a full rebuild, but it is the only way to get package
# updates with out changing the base image.
ARG CACHE_BUST="2022-02-24"

# =============================================================================
# Pull in the version of cargo-chef we plan to use, so that all the below steps
# use a consistent set of versions.
FROM lukemathwalker/cargo-chef:latest-rust-slim-bullseye as chef
WORKDIR /app

# =============================================================================
# Analyze the project, and produce a plan to compile its dependcies. This will
# be run every time. The output should only change if the dependencies of the
# project change, or if significant details of the build process change.

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# =============================================================================
# Use the plan from above to build only the dependencies of the project. This
# should almost always be pulled straight from cache unless dependencies or the
# build process change.

FROM chef as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# =============================================================================
# Now build the project, taking advantage of the cached dependencies from
# above. The version number here should be the same as the version used by cargo-chef
FROM chef as builder
ARG CACHE_BUST

RUN mkdir -m 755 bin
RUN apt-get -qq update && \
  apt-get -qq upgrade
RUN rustup default ${RUST_TOOLCHAIN} && \
  cargo --version && \
  rustc --version
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

RUN cargo build --release --bin swatcher
RUN cp /app/target/release/swatcher /app/bin

# =============================================================================
# Finally prepare a Docker image based on a slim image that only contains the
# files needed to run the project.
FROM debian:bullseye-slim as runtime
ARG CACHE_BUST

RUN apt-get -qq update && \
  apt-get -qq upgrade && \
  apt-get -qq install ca-certificates && \
  (rm -rf /var/lib/apt/lists || true)
RUN groupadd --gid 10001 app && \
  useradd --uid 10001 --gid 10001 --home /app --create-home app

COPY --from=builder /app/bin /app/bin

ENV PORT 3030
WORKDIR /app
USER app
EXPOSE ${PORT}

CMD ["/app/bin/swatcher"]
