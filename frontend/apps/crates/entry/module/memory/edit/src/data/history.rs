use super::raw::*;

#[derive(Clone, Debug, Default)]
pub struct History {
    pub pairs: Vec<(Card, Card)>
}
