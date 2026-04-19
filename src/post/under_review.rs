use super::{DraftPost, PublishedPost};
use crate::ReproveReason;

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
        let rejection_msg = ReproveReason::Generic.message();
        let rejection_msg = format!("{rejection_msg}\n{}", self.content);
        DraftPost::new(&rejection_msg)
    }
}

#[cfg(test)]
mod tests {
    use super::UnderReviewPost;
    use crate::ReproveReason;

    #[test]
    fn new_preserves_content() {
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
    fn approve_preserves_content() {
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
    fn reprove_preserves_content() {
        let post = UnderReviewPost::new("untouched content");
        let post = post.reprove();
        assert!(post.content().contains("untouched content"))
    }

    #[test]
    fn reprove_inserts_message() {
        let post = UnderReviewPost::new("content").reprove();
        let rejection_msg = ReproveReason::Generic.message();
        assert!(
            post.content().starts_with(&rejection_msg),
            "reprove should insert message in content"
        );
    }
}
