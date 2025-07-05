struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: `new` 関数を実装してください。
    //  要件は次のとおりです。
    //   - `status` は `To-Do`, `In Progress`, `Done` のいずれかのみ許可すること。
    //   - `title` と `description` は空文字列であってはならない。
    //   - `title` は 50 バイト以内であること。
    //   - `description` は 500 バイト以内であること。
    //  いずれかの要件を満たさない場合、このメソッドは panic すべきです。
    //  必要な panic メッセージはテスト内に示されています。
    //
    //  これまでの演習で学んだ内容に加えて `String` 型のメソッドを活用してください。
    //  標準ライブラリのドキュメントを参照し、最適な手段を選びましょう:
    //  https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if title.as_bytes().len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.as_bytes().len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        match status.as_str() {
            "To-Do" | "In Progress" | "Done" => {}
            _ => panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed"),
        }

        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
