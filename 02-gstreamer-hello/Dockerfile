# ----------------------------------------------------------------------------
# Ultra-minimal GStreamer-Rust demo (single-stage build)
#   • Installs the Rust toolchain and GStreamer runtime/dev packages
#   • Builds the binary in /usr/local/bin
#   • Keeps all build tools inside the final image (simpler, but larger)
# ----------------------------------------------------------------------------
FROM ubuntu:24.04

# --- 1) OS prerequisites ----------------------------------------------------
RUN apt-get update -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
        build-essential            \
        curl wget ca-certificates  \
        pkg-config                 \
        # --- Core GStreamer & common plugins -------------------------------
        gstreamer1.0-tools         \
        gstreamer1.0-libav         \
        gstreamer1.0-plugins-base  \
        gstreamer1.0-plugins-good  \
        # --- GStreamer development packages (needed for compilation) -------
        libgstreamer1.0-dev        \
        libgstreamer-plugins-base1.0-dev \
        libglib2.0-dev             \
        # To keep the image small(ish) we skip -bad / -ugly here, add them
        # later if you need extra codecs.                                     \
    && rm -rf /var/lib/apt/lists/*

# --- 2) Rust toolchain ------------------------------------------------------
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

# --- 3) Copy source and build ----------------------------------------------
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# --- 4) Install the binary --------------------------------------------------
RUN install -Dm755 target/release/gst_hello /usr/local/bin/gst_hello

# --- 5) Default entrypoint --------------------------------------------------
ENTRYPOINT ["gst_hello"]
# Usage inside the container:
#   docker run --rm -v "$PWD:/data" <image> /data/video.mp4
