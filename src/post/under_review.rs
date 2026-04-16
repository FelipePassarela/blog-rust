use super::{DraftPost, PublishedPost};

pub struct UnderReviewPost {
    content: Box<str>,
}

impl UnderReviewPost {
    pub(super) fn new(content: &str) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn approve(self) -> PublishedPost {
        PublishedPost::new(&self.content)
    }

    pub fn reprove(self) -> DraftPost {
        DraftPost::new(&self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::UnderReviewPost;

    #[test]
    fn new_keeps_content_unchanged() {
        let post = UnderReviewPost::new("untouched content");
        assert_eq!("untouched content", post.content.as_ref());
    }

    #[test]
    fn approve_consumes_self() {
        let takes_ownership: fn(UnderReviewPost) -> _ = UnderReviewPost::approve;
        let post = UnderReviewPost::new("");
        takes_ownership(post);
    }

    #[test]
    fn approve_keeps_content_unchanged() {
        let post = UnderReviewPost::new("untouched content");
        let post = post.approve();
        assert_eq!("untouched content", post.content())
    }

    #[test]
    fn reprove_consumes_self() {
        let takes_ownership: fn(UnderReviewPost) -> _ = UnderReviewPost::reprove;
        let post = UnderReviewPost::new("");
        takes_ownership(post);
    }

    #[test]
    fn reprove_keeps_content_unchanged() {
        let post = UnderReviewPost::new("untouched content");
        let post = post.reprove();
        assert_eq!("untouched content", post.content())
    }
}
