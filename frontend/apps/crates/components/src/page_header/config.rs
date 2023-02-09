use super::PageLinks;

#[derive(Clone, Default)]
pub struct PageHeaderConfig {
    pub slot: Option<&'static str>,
    pub active_page: Option<PageLinks>,
    pub render_beta: bool,
}
