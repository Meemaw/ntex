use ntex::http::{Response, StatusCode, Version};

#[test]
fn response_extensions_are_cleared_when_reused_from_pool() {
    let resp = Response::new(StatusCode::OK);
    resp.extensions_mut().insert(42usize);
    drop(resp);

    let resp = Response::new(StatusCode::OK);
    assert!(resp.extensions().get::<usize>().is_none());
}

#[test]
fn response_version_is_reset_when_reused_from_pool() {
    // a response head released back to the thread-local message pool kept its
    // version; the next response built on the same thread observed the stale
    // value via `head.version` (e.g. h2 `prepare_response` never sets it)
    let mut resp = Response::new(StatusCode::OK);
    resp.head_mut().version = Version::HTTP_2;
    drop(resp);

    let resp = Response::new(StatusCode::OK);
    assert_eq!(resp.head().version, Version::HTTP_11);
}
