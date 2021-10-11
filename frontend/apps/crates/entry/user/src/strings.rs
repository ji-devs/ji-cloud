pub const STR_AUTH_OAUTH_LOGIN_FAIL:&'static str = "Create an account first :)";
pub const STR_CHANGE_EMAIL:&'static str = "Change email account";
pub const STR_SUBMIT:&'static str = "Submit";
pub const STR_CONTINUE:&'static str = "Continue";
pub const STR_EMAIL_LABEL:&'static str = "Email*";
pub const STR_EMAIL_PLACEHOLDER:&'static str = "Type or paste your email";
pub const STR_PASSWORD_CREATE_LABEL:&'static str = "Create password*";
pub const STR_PASSWORD_PLACEHOLDER:&'static str ="********";

pub const STR_NOT_AUTHORIZED:&'static str = "Not authorized!";
pub const STR_USER_EXISTS:&'static str = "User already exists!";
pub const STR_EMPTY_USERNAME:&'static str = "Empty username!";


pub const STR_PASSWORD_LABEL:&'static str = "Password*";
pub const STR_PASSWORD_FORGOTTEN:&'static str ="Forgot your Password?";

pub mod profile {
    pub const STR_FORGOT_PASSWORD: &'static str = "Forgot your password?";
    pub const STR_SAVE: &'static str = "Save";
    pub const STR_CANCEL: &'static str = "Cancel";
    pub const STR_CURRENT_PASSWORD_LABEL:&'static str = "Current password";
    pub const STR_NEW_PASSWORD_LABEL:&'static str = "New password";
    pub const STR_RETYPE_NEW_PASSWORD_LABEL:&'static str = "Retype new password";
    pub const STR_PASSWORD_PLACEHOLDER:&'static str = "Type your password";
}

pub mod register {
    pub mod complete {
        pub const STR_SUBMIT:&'static str = "Go to Jigzi";
    }

    pub mod step_1 {
        pub const STR_FIRSTNAME_LABEL:&'static str = "First name*";
        pub const STR_FIRSTNAME_PLACEHOLDER:&'static str = "Type your first name";
        pub const STR_LASTNAME_LABEL:&'static str = "Last name*";
        pub const STR_LASTNAME_PLACEHOLDER:&'static str = "Type your last name";
        pub const STR_USERNAME_LABEL:&'static str = "Create a User Name*";
        pub const STR_USERNAME_PLACEHOLDER:&'static str = "This will be your public name on Jigzi";
        pub const STR_18:&'static str = "I am over 18*";
        pub const STR_CONTINUE:&'static str = "Continue";
        pub const STR_NEXT:&'static str = "Next";
    }

    pub mod step_2 {
        pub const STR_SUBMIT:&'static str = "Submit";
        pub const STR_LOCATION_LABEL:&'static str = "Location*";
        pub const STR_PERSONA_LABEL:&'static str = "I sign up as a...*";
        pub const STR_ORGANIZATION_LABEL:&'static str = "School/Organization*";
        pub const STR_TERMS_LABEL_ONE:&'static str = "I have read and accept the ";
        pub const STR_TERMS_TERMS:&'static str = "terms & conditions";
        pub const STR_TERMS_LABEL_TWO:&'static str = " and ";
        pub const STR_TERMS_PRIVACY:&'static str = "privacy policy";
        pub const STR_LANGUAGE_LABEL:&'static str = "Language of communication*";
        pub const STR_MARKETING_LABEL:&'static str = "I would like to receive educational resources (GDPR legal text….)";

        pub const STR_LANGUAGE_PLACEHOLDER:&'static str = "Select from the list";
        pub const STR_LANGUAGE_OPTIONS: &'static [&'static str] = &["English", "Hebrew", "Spanish", "French", "Russian"];
        pub const STR_PERSONA_OPTIONS: &'static [&'static str] = &["Teacher", "Parent", "Therapist", "Grandparent", "School leader", "Administrator", "Shaliach", "Tutor"];
        pub const STR_PERSONA_PLACEHOLDER:&'static str = "Select from the list";
        pub const STR_ONE_MORE_STEP:&'static str = "One more step";

    }

    pub mod step_3 {
        pub const STR_SUBMIT:&'static str = "Submit";
        pub const STR_AGE_LABEL:&'static str = "Which age group are you interested in?";
        pub const STR_AFFILIATION_LABEL:&'static str = "Content from which affiliations do you want to see?";
    }

}
