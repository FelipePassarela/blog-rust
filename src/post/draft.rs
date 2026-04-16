use super::UnderReviewPost;

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub(super) fn new(content: &str) -> Self {
        Self {
            content: String::from(content),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn add_text(mut self, text: &str) -> Self {
        self.content += text;
        self
    }

    pub fn request_review(self) -> UnderReviewPost {
        UnderReviewPost::new(&self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::DraftPost;

    #[test]
    fn new_creates_draft_with_content() {
        let post = DraftPost::new("new content");
        assert_eq!("new content", post.content());
    }

    #[test]
    fn add_text_append_to_content() {
        let post = DraftPost::new("appended_").add_text("content");
        assert_eq!("appended_content", post.content());
    }

    #[test]
    fn request_review_consumes_draft() {
        let takes_ownership: fn(DraftPost) -> _ = DraftPost::request_review;
        let post = DraftPost::new("");
        takes_ownership(post);
    }
}
