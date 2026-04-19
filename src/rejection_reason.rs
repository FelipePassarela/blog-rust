pub enum ReproveReason {
    Generic,
    EmptyContent,
}

impl ReproveReason {
    pub fn message(&self) -> &'static str {
        match self {
            ReproveReason::Generic => {
                "[REPROVED] Your post was reproved. Please review the content and \
                submit again for review."
            }
            ReproveReason::EmptyContent => {
                "[REPROVED] Your post was reproved due to empty content. Please add \
                some content and submit again for review."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReproveReason;

    #[test]
    fn message_returns_generic_msg() {
        let reason = ReproveReason::Generic;
        let expected_msg = "[REPROVED] Your post was reproved. Please review the \
            content and submit again for review.";
        assert_eq!(expected_msg, reason.message());
    }

    #[test]
    fn message_returns_empty_content_msg() {
        let reason = ReproveReason::EmptyContent;
        let expected_msg = "[REPROVED] Your post was reproved due to empty content. \
            Please add some content and submit again for review.";
        assert_eq!(expected_msg, reason.message());
    }
}
