use super::DraftPost;

pub(crate) struct Post;

impl Post {
    pub fn new() -> DraftPost {
        DraftPost::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::Post;

    #[test]
    fn new_creates_empty_draft() {
        let post = Post::new();
        assert!(post.content().is_empty())
    }
}
