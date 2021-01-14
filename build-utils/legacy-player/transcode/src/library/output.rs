#[derive(Deserialize, Debug)]
pub enum ActivitySettings {
    Questions(Questions),
    SaySomething(SaySomething),
    Soundboard(Soundboard),
    Video(Video),
    Puzzle(Puzzle),
    TalkType(TalkType),
}



#[derive(Deserialize, Debug)]
pub struct Questions {}


#[derive(Deserialize, Debug)]
pub struct SaySomething {
    pub advance: bool,

    #[serde(rename="linkToPage")]
    pub jump_index: usize,
}


#[derive(Deserialize, Debug)]
pub struct Soundboard {
    #[serde(rename="soundFunModeV2")]
    pub fun_mode: bool,

    #[serde(rename="soundHideHints")]
    pub hide_hints: bool,

    #[serde(rename="kIsShowSoundboardHintsOnStart")]
    pub hints_on_start: bool,

    #[serde(rename="kShowConfetti")]
    pub confetti: bool,
}

#[derive(Deserialize, Debug)]
pub struct Video {
    #[serde(rename="videoRange")]
    pub range: VideoRange,

    #[serde(rename="videoTitle")]
    pub title: String,
    
    #[serde(rename="videoURL")]
    pub url: String,

    pub transform: Transform,

    #[serde(rename="videoThumbURL")]
    pub img_thumb: String, 
}


#[derive(Deserialize, Debug)]
pub struct Puzzle {
    #[serde(rename="linkToPage")]
    pub jump_index: usize,

    #[serde(rename="soundFunModeV2")]
    pub fun_mode: bool,

    #[serde(rename="showShapeV2")]
    pub show_shape: bool,

    #[serde(rename="DisableHints")]
    pub hints_disabled: bool,

    #[serde(rename="ShapePuzzleThemeV2")]
    pub theme: bool,
}

#[derive(Deserialize, Debug)]
pub struct TalkType {
    #[serde(rename="linkToPage")]
    pub jump_index: usize,

    #[serde(rename="soundShowToolTip")]
    pub tooltip: bool,
}
