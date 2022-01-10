#![allow(dead_code)] // this should be removed eventually

pub const STR_AUTH_OAUTH_LOGIN_FAIL: &str = "Create an account first :)";
pub const STR_CHANGE_EMAIL: &str = "Change email account";
pub const STR_SUBMIT: &str = "Submit";
pub const STR_CONTINUE: &str = "Continue";
pub const STR_EMAIL_LABEL: &str = "Email *";
pub const STR_PASSWORD_CREATE_LABEL: &str = "Create password *";
pub const STR_PASSWORD_PLACEHOLDER: &str = "********";

pub const STR_NOT_AUTHORIZED: &str = "Not authorized!";
pub const STR_USER_EXISTS: &str = "User already exists!";
pub const STR_EMPTY_USERNAME: &str = "Empty username!";

pub const STR_PASSWORD_LABEL: &str = "Password *";
pub const STR_PASSWORD_FORGOTTEN: &str = "Forgot your password?";

pub mod profile {
    pub const STR_FORGOT_PASSWORD: &str = "Forgot your password?";
    pub const STR_SAVE: &str = "Save";
    pub const STR_CANCEL: &str = "Cancel";
    pub const STR_CURRENT_PASSWORD_LABEL: &str = "Current password";
    pub const STR_NEW_PASSWORD_LABEL: &str = "New password";
    pub const STR_RETYPE_NEW_PASSWORD_LABEL: &str = "Retype new password";
    pub const STR_PASSWORD_PLACEHOLDER: &str = "Type your password";
}

pub mod register {
    pub mod complete {
        pub const STR_SUBMIT: &str = "Go to Jigzi";
    }

    pub mod step_1 {
        pub const STR_FIRSTNAME_LABEL: &str = "First name *";
        pub const STR_LASTNAME_LABEL: &str = "Last name *";
        pub const STR_USERNAME_LABEL: &str = "Create a username *";
        pub const STR_USERNAME_PLACEHOLDER: &str = "This will be public to all Jigzi members";
        pub const STR_18: &str = "I am over 18 *";
        pub const STR_CONTINUE: &str = "Continue";
        pub const STR_NEXT: &str = "Next";
    }

    pub mod step_2 {
        pub const STR_SUBMIT: &str = "Submit";
        pub const STR_LOCATION_LABEL: &str = "Location *";
        pub const STR_LOCATION_PLACEHOLDER: &str = "City, State";
        pub const STR_PERSONA_LABEL: &str = "I am signing up as a... *";
        pub const STR_ORGANIZATION_LABEL: &str = "School/Organization";
        pub const STR_TERMS_LABEL_ONE: &str = "I have read and accept the ";
        pub const STR_TERMS_TERMS: &str = "terms & conditions";
        pub const STR_TERMS_LABEL_TWO: &str = " and ";
        pub const STR_TERMS_PRIVACY: &str = "privacy policy";
        pub const STR_MARKETING_LABEL: &str = "I would like to receive educational resources.";
        pub const STR_PROTECTING_PRIVACY:&str = "Jewish Interactive (Ji) is committed to protecting and respecting your privacy. We will only use your personal information to administer your account and to provide the products and services you requested from us.";

        pub const STR_LANGUAGE_LABEL: &str = "Language of communication *";
        pub const STR_LANGUAGE_PLACEHOLDER: &str = "Select from the list";

        pub const STR_PERSONA_OPTIONS: &[&str] = &[
            "Teacher",
            "Parent",
            "Therapist",
            "Grandparent",
            "School leader",
            "Administrator",
            "Shaliach",
            "Tutor",
            "Content manager",
        ];
        pub const STR_PERSONA_PLACEHOLDER: &str = "Select from the list";
        pub const STR_ONE_MORE_STEP: &str = "One more step";
    }

    pub mod step_3 {
        pub const STR_SUBMIT: &str = "Submit";
        pub const STR_AGE_LABEL: &str = "Which age group are you interested in?";
        pub const STR_AFFILIATION_LABEL: &str =
            "Content from which affiliations do you want to see?";
    }
}
