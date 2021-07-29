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
    UserDefaultPerms,
    Audio,
    MetaImage,
    MetaAnimation,
    MetaAudio,
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
            Self::UserDefaultPerms => include_str!("../../fixtures/13_user_default_perms.sql"),
            Self::Audio => include_str!("../../fixtures/14_audio.sql"),
            Self::MetaImage => include_str!("../../fixtures/15_meta_kinds_image.sql"),
            Self::MetaAnimation => include_str!("../../fixtures/16_meta_kinds_animation.sql"),
            Self::MetaAudio => include_str!("../../fixtures/17_meta_kinds_audio.sql"),
        }
    }
}
