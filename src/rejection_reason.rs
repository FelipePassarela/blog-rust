pub enum RejectionReason {
    Generic,
    EmptyContent,
}

impl RejectionReason {
    pub fn message(&self) -> &'static str {
        match self {
            RejectionReason::Generic => {
                "Your post was rejected. Please review the content and submit again \
                for review."
            }
            RejectionReason::EmptyContent => {
                "Your post was rejected due to empty content. Please add some content \
                and submit again for review."
            }
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            RejectionReason::Generic => "[REPROVED]",
            RejectionReason::EmptyContent => "[REPROVED]",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RejectionReason;

    #[test]
    fn message_returns_generic_msg() {
        let reason = RejectionReason::Generic;
        let expected_msg = "Your post was rejected. Please review the content and \
            submit again for review.";
        assert_eq!(expected_msg, reason.message());
    }

    #[test]
    fn message_returns_empty_content_msg() {
        let reason = RejectionReason::EmptyContent;
        let expected_msg = "Your post was rejected due to empty content. Please add \
            some content and submit again for review.";
        assert_eq!(expected_msg, reason.message());
    }

    #[test]
    fn tag_returns_rejection_tag() {
        assert_eq!("[REPROVED]", RejectionReason::Generic.tag());
        assert_eq!("[REPROVED]", RejectionReason::EmptyContent.tag());
    }
}
