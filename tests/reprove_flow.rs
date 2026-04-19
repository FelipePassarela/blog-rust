use blog_rust::post;

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

    let expected_msg = "[Reproved]\n\n\
        Your post was reproved. Please review the content and submit again for review.";

    assert!(
        post.content().starts_with(expected_msg),
        "reprove flow should insert message in content"
    );
    assert!(
        post.content().contains("content"),
        "reprove flow should preserve original content"
    );
}
