import {renderTemplate as tmpl} from "@utils/template";

import {appendId, addClassesId, setTextId, setValueId, setAttributeId} from "@utils/dom";
import gotEmailConfirmation from "@templates/user/misc/got-email-confirmation.html";
import sendEmailConfirmation from "@templates/user/misc/send-email-confirmation.html";
import forgotPassword from "@templates/user/misc/forgot-password.html";

export default {
  title: 'User/Misc',
}

export const SendEmailConfirmationNotification = () => {
    const page = tmpl(sendEmailConfirmation);

    addClassesId(page, "resend-email", ["hidden"]);
    return page;
}
export const SendEmailConfirmationDone = () => {
    const page = tmpl(sendEmailConfirmation);
    addClassesId(page, "sent-notification", ["hidden"]);
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