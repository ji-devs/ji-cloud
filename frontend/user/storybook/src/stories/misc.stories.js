import {renderTemplate as tmpl} from "@utils/template";
import emailConfirmation from "@templates/email-confirmation.html";
import emailChange from "@templates/email-change.html";
import forgotPassword from "@templates/forgot-password.html";

export default {
  title: 'Misc',
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