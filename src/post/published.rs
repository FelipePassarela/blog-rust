use super::DraftPost;

pub struct PublishedPost {
    content: Box<str>,
}

impl PublishedPost {
    pub(super) fn new(content: &str) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn retract(self) -> DraftPost {
        DraftPost::new(&self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::PublishedPost;

    #[test]
    fn new_preserves_content() {
        let post = PublishedPost::new("untouched content");
        assert_eq!("untouched content", post.content());
    }

    #[test]
    fn retract_preserves_content() {
        let post = PublishedPost::new("untouched content");
        let post = post.retract();
        assert_eq!("untouched content", post.content());
    }

    #[test]
    fn retract_consumes_self() {
        let takes_ownership: fn(PublishedPost) -> _ = PublishedPost::retract;
        let post = PublishedPost::new("");
        takes_ownership(post);
    }
}
