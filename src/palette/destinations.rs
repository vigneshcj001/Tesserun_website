#[derive(Clone, Copy)]
pub struct Destination {
    pub name: &'static str,
    pub description: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
    keywords: &'static str,
}

const DESTINATIONS: [Destination; 5] = [
    Destination {
        name: "Workspace",
        description: "Try the interactive studio",
        href: "#workspace",
        icon: "⌘",
        keywords: "demo playground studio idea agents",
    },
    Destination {
        name: "How it works",
        description: "Explore development, review, and publishing",
        href: "#workflow",
        icon: "✳",
        keywords: "process develop test deploy steps",
    },
    Destination {
        name: "Model providers",
        description: "Find the connection that fits your workflow",
        href: "#models",
        icon: "◉",
        keywords: "ai models openai anthropic gemini groq ollama local",
    },
    Destination {
        name: "Download",
        description: "Get Tesserun for Windows and see the setup guide",
        href: "#download",
        icon: "↓",
        keywords: "install windows exe setup release portable",
    },
    Destination {
        name: "Questions",
        description: "Get answers before your first project",
        href: "#questions",
        icon: "?",
        keywords: "faq faqs help answers",
    },
];

pub fn matching(query: &str) -> Vec<Destination> {
    let query = query.trim().to_lowercase();
    DESTINATIONS
        .into_iter()
        .filter(|item| {
            format!(
                "{} {} {} {}",
                item.name, item.description, item.href, item.keywords
            )
            .to_lowercase()
            .contains(&query)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::matching;

    #[test]
    fn common_aliases_find_the_expected_section() {
        for (query, href) in [
            ("models", "#models"),
            ("local", "#models"),
            (" INSTALL ", "#download"),
            ("FAQ", "#questions"),
        ] {
            assert!(matching(query).iter().any(|item| item.href == href));
        }
    }

    #[test]
    fn empty_and_unmatched_queries_have_predictable_results() {
        assert_eq!(matching("").len(), 5);
        assert!(matching("nothing-matches-9qzx").is_empty());
    }
}
