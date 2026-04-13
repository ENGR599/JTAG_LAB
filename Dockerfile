from ubuntu:22.04

arg DEBIAN_FRONTEND=noninteractive
run --mount=type=cache,sharing=locked,target=/var/lib/apt \
    --mount=type=cache,sharing=locked,target=/var/cache/apt \
: \
&& rm -f /etc/apt/apt.conf.d/docker-clean \
&& echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache \
&& apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  cargo \
  pkg-config \
  libusb-1.0-0-dev \
&& update-ca-certificates

copy . .
run cargo build
