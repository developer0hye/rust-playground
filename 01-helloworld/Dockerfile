FROM ubuntu:24.04

# Install necessary dependencies
RUN apt-get update && apt-get install -y curl build-essential

# Install Rust using rustup, the official toolchain installer.
#
# We are using a pipe `|` to send the downloaded script directly to the shell `sh` for execution.
#
# curl arguments:
# --proto '=https': Enforces that the connection must be HTTPS. The '=' makes it apply even after redirects.
#                  This is a security measure to ensure the script is downloaded over a secure channel.
# --tlsv1.2:       Forces the use of TLS version 1.2 for the connection, preventing downgrade attacks to older,
#                  less secure TLS versions.
# -s:              Silent mode. Don't show a progress meter.
# -S:              Show error message on error, even if in silent mode.
# -f:              Fail fast. Exit with an error code on server errors (like 404) instead of outputting the error page.
#
# sh arguments:
# -s:   Tells the shell to read the script from standard input (which comes from the curl pipe).
# --:   A special argument that signifies the end of options for `sh`.
# -y:   This argument is passed to the rustup-init script itself. It means "yes" to all prompts,
#       accepting the default installation options without user interaction, which is crucial for Docker builds.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add cargo to PATH so we can use `rustc`, `cargo`, etc. in subsequent RUN/CMD instructions.
# The rustup script installs them in /root/.cargo/bin by default.
ENV PATH="/root/.cargo/bin:${PATH}"

# Set a working directory
WORKDIR /app

# Copy the source code
COPY main.rs .

# Compile the source code
RUN rustc main.rs

# Set the command to run the executable.
# We use `sh -c` to run multiple commands in sequence:
# 1. Print the version of the Rust compiler.
# 2. Print the version of Cargo.
# 3. Execute the compiled application.
CMD ["sh", "-c", "rustc --version && cargo --version && echo '---' && ./main"] 