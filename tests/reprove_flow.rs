use blog_rust::post;

#[test]
fn reprove_flow_preserves_content() {
    let post = post::Post::new()
        .add_text("untouched content")
        .request_review()
        .reprove();

    assert_eq!(
        "untouched content",
        post.content(),
        "reprove flow should preserve content"
    );
}
