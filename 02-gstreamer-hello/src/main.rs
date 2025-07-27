//! A *bare-bones* GStreamer example written in Rust.
//!
//! What it does
//! ============
//! 1. Initialise the GStreamer runtime
//! 2. Convert a CLI **file path** into a canonical `file://` URI
//! 3. Build a `playbin` element and point it at that URI
//! 4. Start playback (headless - no video/audio output)
//! 5. Block the main thread on the Bus until either
//!      • End-of-Stream (EOS) -- normal termination, or
//!      • Error -- something went wrong
//! 6. Tear everything down cleanly and exit
//!
//! Build (native):
//!     cargo build --release
//!
//! Run (native):
//!     ./target/release/gst_hello ./clip.mp4
//!
//! Run in Docker (after building the image):
//!     docker run --rm -v "$PWD:/data" gst-hello /data/clip.mp4
//!     # ^ Adjust the image/tag if you changed it
//!
//! Only *one* dependency (gstreamer) is needed.

use gstreamer as gst;
use gstreamer::prelude::*;
use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // -----------------------------------------------------------
    // 1) Start up the GStreamer core library.
    //    Any failure here usually means the system can't find
    //    plugins or has incompatible versions.
    // -----------------------------------------------------------
    gst::init()?;

    // -----------------------------------------------------------
    // 2) Grab the first CLI argument and turn it into a URI.
    //    Convert file path to proper file:// URI manually
    // -----------------------------------------------------------
    let file_path = env::args()
        .nth(1)
        .expect("Usage: gst_hello <video-file>");
    
    let path = Path::new(&file_path);
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };
    
    let uri = format!("file://{}", absolute_path.to_string_lossy());

    // -----------------------------------------------------------
    // 3) Create a playbin element configured for headless operation
    //    This works better in Docker containers without display/audio
    // -----------------------------------------------------------
    let playbin = gst::ElementFactory::make("playbin")
        .build()
        .expect("Unable to create playbin element");
    
    // Set the URI
    playbin.set_property_from_str("uri", &uri);
    
    // Create fake sinks for headless operation
    let video_sink = gst::ElementFactory::make("fakesink")
        .build()
        .expect("Unable to create video fakesink");
    
    let audio_sink = gst::ElementFactory::make("fakesink")
        .build()
        .expect("Unable to create audio fakesink");
    
    // Configure the playbin to use fake sinks (no actual output)
    playbin.set_property("video-sink", &video_sink);
    playbin.set_property("audio-sink", &audio_sink);

    println!("🎬 Starting playback of: {}", file_path);
    println!("📺 Running in headless mode (no video/audio output)");

    // -----------------------------------------------------------
    // 4) Kick off playback.
    // -----------------------------------------------------------
    playbin.set_state(gst::State::Playing)?;

    // -----------------------------------------------------------
    // 5) Classical *Bus* loop.
    //    The Bus transports messages (EOS, errors, tags, etc.)
    //    from the internal streaming threads back to *this*
    //    thread so we can react.
    // -----------------------------------------------------------
    let bus = playbin
        .bus()
        .expect("Pipeline without a Bus — should never happen");

    // Print some info about the media
    println!("⏯️  Playback started... waiting for completion");

    // Block until we receive EOS (normal) or Error (abnormal).
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView::*;
        match msg.view() {
            Eos(..) => {
                println!("✅ End-of-Stream reached.");
                break;
            }
            Error(err) => {
                eprintln!(
                    "❌ Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            StateChanged(state_changed) => {
                if state_changed.src().map(|s| s == &playbin).unwrap_or(false) {
                    println!("🔄 State changed: {:?} -> {:?}", 
                        state_changed.old(), 
                        state_changed.current()
                    );
                }
            }
            _ => {} // Ignore everything else for this minimal demo
        }
    }

    // -----------------------------------------------------------
    // 6) Always return the pipeline to the NULL state so that
    //    all resources (files, devices, threads) are released.
    // -----------------------------------------------------------
    playbin.set_state(gst::State::Null)?;
    println!("🔚 Pipeline shut down. Bye!");
    Ok(())
}
