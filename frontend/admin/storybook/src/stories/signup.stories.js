import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import login from "@templates/login/login.html";

export default {
  title: 'Sign up',
}

export const Login = () =>
    tmpl(login, {

});
