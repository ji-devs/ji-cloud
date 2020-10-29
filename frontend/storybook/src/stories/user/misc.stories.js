import {renderTemplate as tmpl} from "@utils/template";
import gotEmailConfirmation from "@templates/user/misc/got-email-confirmation.html";
import sendEmailConfirmation from "@templates/user/misc/send-email-confirmation.html";
import forgotPassword from "@templates/user/misc/forgot-password.html";

export default {
  title: 'User/Misc',
}

export const SendEmailConfirmation = () => {
    const page = tmpl(sendEmailConfirmation);
    return page;
}
export const GotEmailConfirmation = () => {
    const page = tmpl(gotEmailConfirmation);
    return page;
}


export const ForgotPassword = () => {
    const page = tmpl(forgotPassword);
    return page;
}