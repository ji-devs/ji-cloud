#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModulePageKind {
    Empty,
    GridPlain,
    GridResize,
    GridResizeScrollable,
    GridResizePreview,
    Iframe,
}

impl ModulePageKind {
    pub fn is_resize(&self) -> bool {
        match self {
            Self::GridResize
            | Self::GridResizeScrollable
            | Self::GridResizePreview
            | Self::Iframe => true,
            _ => false,
        }
    }
    pub fn add_scrollable_attribute(&self) -> bool {
        match self {
            Self::GridResizeScrollable => true,
            _ => false,
        }
    }

    pub fn add_preview_attribute(&self) -> bool {
        match self {
            Self::GridResizePreview => true,
            _ => false,
        }
    }
    pub fn element_name(&self) -> &str {
        match self {
            Self::GridResize => "module-page-grid-resize",
            Self::GridResizeScrollable => "module-page-grid-resize",
            Self::GridResizePreview => "module-page-grid-resize",
            Self::GridPlain => "module-page-grid-plain",
            Self::Iframe => "module-page-iframe",
            Self::Empty => "div",
        }
    }
}
