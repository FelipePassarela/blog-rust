use blog_rust::{ReproveReason, post};

#[test]
fn reprove_flow_preserves_content() {
    let post = post::Post::new()
        .add_text("untouched content")
        .request_review()
        .reprove();
    assert!(post.content().ends_with("untouched content"))
}

#[test]
fn reprove_flow_insert_message_in_content() {
    let post = post::Post::new()
        .add_text("content")
        .request_review()
        .reprove();
    let rejection_msg = ReproveReason::Generic.message();

    assert!(
        post.content().starts_with(rejection_msg),
        "reprove flow should insert rejection message in content"
    );
    assert!(
        post.content().contains("content"),
        "reprove flow should preserve original content"
    );
}
