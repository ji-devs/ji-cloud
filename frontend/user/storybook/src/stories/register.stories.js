import {renderTemplate as tmpl} from "@utils/template";
import register1 from "@templates/register/register-1.html";
import register2 from "@templates/register/register-2.html";
import register3 from "@templates/register/register-3.html";
import registerSentEmail from "@templates/register/register-sent-email.html";
import registerFinal from "@templates/register/register-final.html";
import registergoogle from "@templates/register/register-google.html";

export default {
  title: 'Register',
}

export const Step1 = () => {
    const page = tmpl(register1);
    return page;
}

export const RegisterGoogle = () => {
    const page = tmpl(registergoogle);
    return page;
}

export const Step2 = () => {
    const page = tmpl(register2);
    return page;
}

export const Step3 = () => {
    const page = tmpl(register3);
    return page;
}

export const SentEmail = () => {
    const page = tmpl(registerSentEmail);
    return page;
}

export const Final = () => {
    const page = tmpl(registerFinal);
    return page;
}
