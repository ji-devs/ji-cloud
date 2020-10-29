import {renderTemplate as tmpl} from "@utils/template";
import register1 from "@templates/user/register/register-1.html";
import register2 from "@templates/user/register/register-2.html";
import register3 from "@templates/user/register/register-3.html";
import registerFinal from "@templates/user/register/register-final.html";
import registerStart from "@templates/user/register/register-start.html";

export default {
  title: 'User/Register',
}

export const Start = () => {
    const page = tmpl(registerStart);
    return page;
}

export const Step1 = () => {
    const page = tmpl(register1);
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

export const Final = () => {
    const page = tmpl(registerFinal);
    return page;
}
