FROM openfaas/of-watchdog:0.8.2 as watchdog

FROM rust:1.51 as builder

WORKDIR /home/rust

# Copy all the sources
COPY function ./function

# Useful when we are working with Cargo workspaces
# and we need to define which binary implements our
# function
ARG HANDLER_PACKAGE=function

RUN cd function && cargo build --release --bin $HANDLER_PACKAGE

FROM ubuntu:xenial as runner

ARG HANDLER_PACKAGE=function

ENV USER=app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

COPY --from=builder /home/rust/function/target/release/$HANDLER_PACKAGE /usr/bin/function
RUN chmod +x /usr/bin/function

# Set up watchdog for HTTP mode
ENV fprocess="function"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
