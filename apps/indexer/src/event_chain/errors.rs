use anyhow::Error;

pub(super) fn looks_like_range_too_large(err: &Error) -> bool {
    let msg = err.to_string().to_lowercase();
    msg.contains("range too large")
        || msg.contains("block range")
        || msg.contains("too many results")
        || msg.contains("response size exceeded")
        || msg.contains("payload too large")
}

pub(super) fn looks_like_transient(err: &Error) -> bool {
    let msg = err.to_string().to_lowercase();
    msg.contains("timeout")
        || msg.contains("timed out")
        || msg.contains("deadline")
        || (msg.contains("block") && msg.contains("not found"))
        || msg.contains("too many requests")
        || msg.contains("rate limit")
        || msg.contains("429")
        || msg.contains("bad gateway")
        || msg.contains("gateway")
        || msg.contains("service unavailable")
        || msg.contains("503")
        || msg.contains("502")
        || msg.contains("504")
        || msg.contains("connection reset")
        || msg.contains("connection refused")
        || msg.contains("broken pipe")
        || msg.contains("temporarily unavailable")
}
