use dominator::Dom;

use super::state::Sticker;

// http://localhost:4104/module/legacy/play/debug?game_id=17736&slide_index=0&example=true
impl Sticker {
    pub fn render(self) -> Dom {
        match self {
            Self::Image(state) => {
                state.render()
            },
            Self::Animation(state) => {
                state.render()
            },
        }
    }
}
