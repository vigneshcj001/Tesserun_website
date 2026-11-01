#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    Habits,
    Launch,
    Team,
}

impl Scenario {
    pub const ALL: [Self; 3] = [Self::Habits, Self::Launch, Self::Team];
    pub fn label(self) -> &'static str {
        match self {
            Self::Habits => "Habit tracker",
            Self::Launch => "Product launch",
            Self::Team => "Team hub",
        }
    }
    pub fn brief(self) -> &'static str {
        match self {
            Self::Habits => {
                "A calm dashboard for tracking my daily habits. Show a weekly overview, with a little room to grow."
            }
            Self::Launch => {
                "A considered landing page for a new product. Lead with the story, explain the features, and invite people to explore."
            }
            Self::Team => {
                "A shared project hub for a small team. Bring tasks, project notes, and release links into one clear view."
            }
        }
    }
    pub fn plan(self) -> [&'static str; 3] {
        match self {
            Self::Habits => [
                "Define a habit and a daily check-in",
                "Design the weekly overview",
                "Check labels, empty states, and keyboard access",
            ],
            Self::Launch => [
                "Find the product's central story",
                "Build the hero, features, and next step",
                "Check responsive layouts and meaningful links",
            ],
            Self::Team => [
                "Define projects, tasks, and notes",
                "Build a clear project overview",
                "Check navigation and readable status labels",
            ],
        }
    }
    pub fn markup(self) -> &'static str {
        match self {
            Self::Habits => {
                "<section aria-labelledby=\"habits-title\">\n  <h1 id=\"habits-title\">A little progress, every day.</h1>\n  <button aria-pressed=\"false\">Morning walk</button>\n</section>"
            }
            Self::Launch => {
                "<main>\n  <h1>A new way to make your next idea real.</h1>\n  <p>Thoughtful tools. A little more possibility.</p>\n  <a href=\"#features\">Explore the product</a>\n</main>"
            }
            Self::Team => {
                "<section aria-labelledby=\"projects-title\">\n  <h1 id=\"projects-title\">Your team's next chapter.</h1>\n  <nav aria-label=\"Projects\">\n    <a href=\"#release\">Next release</a>\n  </nav>\n</section>"
            }
        }
    }
}
