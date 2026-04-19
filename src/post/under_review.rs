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
        let rejection_message = String::from(
            "[Reproved]\n\n\
            Your post was reproved. Please review the content and submit again for review.",
        );
        let rejected_content = format!("{}{}", rejection_message, self.content);
        DraftPost::new(&rejected_content)
    }
}

#[cfg(test)]
mod tests {
    use super::UnderReviewPost;

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
        let post = UnderReviewPost::new("content");
        let post = post.reprove();
        let expected_msg = "[Reproved]\n\n\
            Your post was reproved. Please review the content and submit again for review.";

        assert!(
            post.content().starts_with(expected_msg),
            "reprove should insert message in content"
        );
    }
}
