# Rust Hello World Docker Example

This directory contains a `Dockerfile` to build a simple Rust "Hello, World!" application.

## Prerequisites

- Docker installed on your system.

## How to Build and Run

1.  **Build the Docker image:**

    Open your terminal in the `01-Dockerfile` directory and run the following command. This will create an image named `rust-hello-world`.

    ```bash
    docker build -t rust-hello-world .
    ```

2.  **Run the Docker container:**

    After the build is complete, run the following command to create and start a container from the image. It will execute the compiled Rust program and print "Hello, world!" to the console.

    ```bash
    docker run --rm rust-hello-world
    ```

    The `--rm` flag automatically removes the container when it exits. 