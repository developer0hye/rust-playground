# GStreamer Hello World - Rust Edition

A minimal GStreamer-based video player written in Rust that demonstrates headless video playback in a Docker container.

## Overview

This project showcases:
- GStreamer integration with Rust using the `gstreamer` crate
- Headless video playback (no audio/video output)
- Containerized deployment with Docker
- Volume mounting for external media files

The application reads a video file and processes it using GStreamer's `playbin` element with fake sinks, making it perfect for video analysis, transcoding pipelines, or CI/CD environments where no display is available.

## Project Structure

```
02-hello-gstreamer/
├── Cargo.toml          # Rust project configuration
├── Dockerfile          # Container build instructions
├── README.md           # This file
├── data/
│   └── sample.mp4      # Sample video file for testing
└── src/
    └── main.rs         # Main application code
```

## Prerequisites

- Docker installed on your system
- Basic familiarity with Docker commands

## Building the Docker Image

Build the Docker image with the following command:

```bash
docker build -t gst-hello .
```

This will:
1. Install Ubuntu 24.04 base system
2. Install GStreamer runtime and development packages
3. Install Rust toolchain
4. Build the Rust application
5. Set up the container entrypoint

## Running the Application

### Using the Sample Video

Run the application with the provided sample video:

```bash
docker run --rm -v "$PWD/data:/data" gst-hello /data/sample.mp4
```

### Using Your Own Video Files

Place your video files in the `data/` directory and run:

```bash
docker run --rm -v "$PWD/data:/data" gst-hello /data/your-video.mp4
```

### Command Breakdown

- `docker run`: Execute a new container
- `--rm`: Automatically remove the container when it exits
- `-v "$PWD/data:/data"`: Mount the local `data/` directory to `/data` inside the container
- `gst-hello`: The Docker image name
- `/data/sample.mp4`: The video file path inside the container (passed as argument to the application)

## How It Works

1. **Initialization**: The application initializes the GStreamer runtime
2. **URI Conversion**: Converts the input file path to a proper `file://` URI
3. **Pipeline Setup**: Creates a `playbin` element with fake audio/video sinks for headless operation
4. **Playback**: Starts playback and monitors the message bus for completion or errors
5. **Cleanup**: Properly shuts down the pipeline and releases resources

## Application Output

When running successfully, you'll see output similar to:

```
🎬 Starting playback of: /data/sample.mp4
📺 Running in headless mode (no video/audio output)
⏯️  Playback started... waiting for completion
🔄 State changed: Null -> Ready
🔄 State changed: Ready -> Paused
🔄 State changed: Paused -> Playing
✅ End-of-Stream reached.
🔚 Pipeline shut down. Bye!
```

## Supported Video Formats

The application supports common video formats through GStreamer plugins:
- MP4 (H.264, H.265)
- AVI
- MOV
- WebM
- And many others supported by GStreamer

## Development

### Building Locally (without Docker)

If you have Rust and GStreamer development packages installed locally:

```bash
cargo build --release
./target/release/gst_hello ./data/sample.mp4
```

### Dependencies

- **gstreamer**: Rust bindings for GStreamer (version 0.24)

## Troubleshooting

### Common Issues

1. **"No such file or directory"**: Ensure the video file exists in the mounted path
2. **GStreamer initialization failed**: The Docker image includes all necessary GStreamer plugins
3. **Permission denied**: Check that the video file has read permissions

### Adding More GStreamer Plugins

To support additional video formats, modify the Dockerfile to include more plugin packages:

```dockerfile
gstreamer1.0-plugins-bad   \
gstreamer1.0-plugins-ugly  \
```

## License

MIT License - see LICENSE file for details.

## Author

Yonghye Kwon <developer.0hye@gmail.com> 