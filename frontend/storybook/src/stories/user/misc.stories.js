import {renderTemplate as tmpl} from "@utils/template";
import emailConfirmation from "@templates/user/email-confirmation.html";
import emailChange from "@templates/user/email-change.html";
import forgotPassword from "@templates/user/forgot-password.html";

export default {
  title: 'User/Misc',
}

export const EmailConfirmation = () => {
    const page = tmpl(emailConfirmation);
    return page;
}

export const EmailChange = () => {
    const page = tmpl(emailChange);
    return page;
}

export const ForgotPassword = () => {
    const page = tmpl(forgotPassword);
    return page;
}