import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId, setTextId} from "@utils/dom";
import signIn from "@templates/signin_and_registration/signin.html";
import register from "@templates/signin_and_registration/register.html";

export default {
  title: 'Signin and Registration',
}

export const SignIn = () => tmpl(signIn, {});
export const Register = () => tmpl(register, {});