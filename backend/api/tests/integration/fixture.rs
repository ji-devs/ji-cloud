#[derive(Debug, Copy, Clone)]
pub enum Fixture {
    User,
    MetaKinds,
    CategoryOrdering,
    CategoryNesting,
    Image,
    UserNoPerms,
    Locale,
    Jig,
    Animation,
    UserColors,
    UserFonts,
}

impl Fixture {
    pub const fn as_query(self) -> &'static str {
        match self {
            Self::User => include_str!("../../fixtures/1_user.sql"),
            Self::MetaKinds => include_str!("../../fixtures/2_meta_kinds.sql"),
            Self::CategoryOrdering => include_str!("../../fixtures/3_category_ordering.sql"),
            Self::CategoryNesting => include_str!("../../fixtures/4_category_nesting.sql"),
            Self::Image => include_str!("../../fixtures/5_image.sql"),
            Self::UserNoPerms => include_str!("../../fixtures/6_user_no_perms.sql"),
            Self::Locale => include_str!("../../fixtures/7_locale.sql"),
            Self::Jig => include_str!("../../fixtures/8_jig.sql"),
            Self::Animation => include_str!("../../fixtures/9_animation.sql"),
            Self::UserColors => include_str!("../../fixtures/11_user_color.sql"),
            Self::UserFonts => include_str!("../../fixtures/12_user_font.sql"),
        }
    }
}
