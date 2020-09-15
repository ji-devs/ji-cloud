import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import login from "@templates/login/login.html";
import forgotpassword from "@templates/login/forgot.html";
import signupone from "@templates/login/signupone.html";
import signuptwo from "@templates/login/signuptwo.html";
import signupthree from "@templates/login/signupthree.html";
import signupfour from "@templates/login/signupfour.html";
import schoolfilter from "@templates/login/schoolfilter.html";
import confirmation from "@templates/login/confirmation.html";
import final from "@templates/login/final.html";
import newemail from "@templates/login/newemail.html";

export default {
  title: 'Sign up',
}

export const Login = () =>
    tmpl(login, {

});

export const ForgotPassword = () =>
    tmpl(forgotpassword, {

});

export const SignUpOne = () =>
    tmpl(signupone, {

});

export const SignUpTwo = () =>
    tmpl(signuptwo, {

});

export const SignUpThree = () =>
    tmpl(signupthree, {

});

export const SignUpFour = () =>
    tmpl(signupfour, {

});

export const Confirmation = () =>
    tmpl(confirmation, {

});

export const NewEmail = () =>
    tmpl(newemail, {

});

export const Final = () =>
    tmpl(final, {

});

export const SchoolFilter = () =>  {
    const pageContainer = tmpl(signuptwo);

    const pageContents = tmpl(schoolfilter);

    appendId(pageContainer, "school-filter", pageContents);

    return pageContainer;
}
