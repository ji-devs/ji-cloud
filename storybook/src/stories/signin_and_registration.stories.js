import {story, storyAbout} from "@utils/stories";
import {signIn} from "@html/signin_and_registration/signin";
import {register} from "@html/signin_and_registration/register";

export default {
  title: 'Signin and Registration',
}

export const SignIn = storyAbout(
    "sign in", 
    signIn, 
    `## Signin page 
        Reached from anywhere 
    `
);


export const Register = storyAbout(
    "register", 
    register, 
    `## Registration page 
        Reached from anywhere 
    `
);
