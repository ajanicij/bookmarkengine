use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub enum Item {
    Folder{name: String},
    Bookmark{ description: String, path: String, href: String, last_modified: DateTime<Utc>, },
    Unfolder, // go up to parent folder
}

impl Item {
    pub fn message(&self) -> String {
        let res: String = match self {
            Item::Folder{name} => format!("Folder {}", name),
            Item::Bookmark{description: _, path: _, href, last_modified: _ } => {
                format!("Bookmark {}", href)
            },
            Item::Unfolder => format!("Go up one folder"),
        };

        // Truncate to maximum length, ending with "..." if we must.
        let max_len = 40;
        if res.chars().count() <= max_len {
            res.clone()
        } else {
            let truncated: String = res.chars().take(max_len.saturating_sub(3)).collect();
            format!("{}...", truncated)
        }
    }
}