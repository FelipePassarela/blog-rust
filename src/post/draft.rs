use super::UnderReviewPost;

pub(crate) struct DraftPost {
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

    pub fn add_text(&mut self, text: &str) {
        self.content += text;
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
        let draft = DraftPost::new("test content");
        assert_eq!("test content", draft.content());
    }

    #[test]
    fn content_returns_stored_content() {
        let draft = DraftPost::new("test content");
        assert_eq!("test content", draft.content());
    }

    #[test]
    fn add_text_append_to_content() {
        let mut draft = DraftPost::new("test_");
        draft.add_text("content");
        assert_eq!("test_content", draft.content());
    }

    #[test]
    fn request_review_consumes_draft() {
        let takes_ownership: fn(DraftPost) -> _ = DraftPost::request_review;
        let draft = DraftPost::new("");
        takes_ownership(draft);
    }
}
