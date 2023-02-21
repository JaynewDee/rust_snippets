use tracing::{event, span, Level};

// records an event outside of any span context:
fn main() {
    event!(Level::INFO, "something happened");

    let span = span!(Level::INFO, "my_span");
    let _guard = span.enter();

    // records an event within "my_span".
    event!(Level::DEBUG, "something happened inside my_span");
}
