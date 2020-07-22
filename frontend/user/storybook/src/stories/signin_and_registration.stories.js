import {story, storyAbout} from "@utils/stories";
import {renderTemplate} from "@common/js/render";
import signIn from "@templates/signin_and_registration/signin.html";
import register from "@templates/signin_and_registration/register.html";

export default {
  title: 'Signin and Registration',
}

export const SignIn = storyAbout(
    "sign in", 
    () => renderTemplate(signIn), 
    `## Signin page 
        Reached from anywhere 
    `
);


export const Register = storyAbout(
    "register", 
    () => renderTemplate(register), 
    `## Registration page 
        Reached from anywhere 
    `
);
