import {renderTemplate as tmpl} from "@utils/template";
import signin from "@templates/user/signin/signin.html";

export default {
  title: 'User/Sign In',
}

export const Signin = () =>
    tmpl(signin, {});