// These must match the typescript / custom element variants

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineKind {
    CardView,
    GameDisplay,
    Hint,
    Next,
    Rounds,
    TimeLimit,
    Attempts,
    Score,
    VideoPlay,
    VideoFeatures,
}

impl LineKind {
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::CardView => "card-view",
            Self::GameDisplay => "game-display",
            Self::Rounds => "rounds",
            Self::Hint => "hint",
            Self::Next => "next",
            Self::TimeLimit => "time-limit",
            Self::Attempts => "attempts",
            Self::Score => "score",
            Self::VideoPlay => "video-play",
            Self::VideoFeatures => "video-features",
        }
    }
}
