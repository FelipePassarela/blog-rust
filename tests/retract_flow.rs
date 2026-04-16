use blog_rust::post;

#[test]
fn retract_flow_preserves_content() {
    let post = post::Post::new()
        .add_text("untouched content")
        .request_review()
        .approve()
        .retract();

    assert_eq!(
        "untouched content",
        post.content(),
        "content should be preserved after retract flow"
    )
}
