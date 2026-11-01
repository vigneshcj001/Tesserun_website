#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Develop,
    Check,
    Deploy,
}

impl Step {
    pub const ALL: [Self; 3] = [Self::Develop, Self::Check, Self::Deploy];
    pub fn tab_id(self) -> &'static str {
        match self {
            Self::Develop => "tab-develop",
            Self::Check => "tab-check",
            Self::Deploy => "tab-deploy",
        }
    }
    pub fn number(self) -> &'static str {
        match self {
            Self::Develop => "01",
            Self::Check => "02",
            Self::Deploy => "03",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Develop => "Develop",
            Self::Check => "Review",
            Self::Deploy => "Ship",
        }
    }
    pub fn breadcrumb(self) -> &'static str {
        match self {
            Self::Develop => "Development",
            Self::Check => "Quality checks",
            Self::Deploy => "Deployments",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            Self::Develop => "⌘",
            Self::Check => "◉",
            Self::Deploy => "↗",
        }
    }
    pub fn for_key(self, key: &str) -> Option<Self> {
        let index = match self {
            Self::Develop => 0,
            Self::Check => 1,
            Self::Deploy => 2,
        };
        match key {
            "ArrowRight" => Some(Self::ALL[(index + 1) % 3]),
            "ArrowLeft" => Some(Self::ALL[(index + 2) % 3]),
            "Home" => Some(Self::Develop),
            "End" => Some(Self::Deploy),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Step;

    #[test]
    fn keyboard_navigation_wraps_and_ignores_unrelated_keys() {
        assert_eq!(Step::Deploy.for_key("ArrowRight"), Some(Step::Develop));
        assert_eq!(Step::Develop.for_key("ArrowLeft"), Some(Step::Deploy));
        assert_eq!(Step::Check.for_key("Home"), Some(Step::Develop));
        assert_eq!(Step::Check.for_key("End"), Some(Step::Deploy));
        assert_eq!(Step::Check.for_key("Tab"), None);
    }
}
