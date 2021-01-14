import "@elements/admin/templates-layout/signup-full-wide";
import "@elements/titles/subtitle";
import "@elements/titles/plain-blue";
import "@elements/dividers/spacer-fourty";
import "@elements/contact";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
  
  }

  const DEFAULT_ARGS:LoginArgs = {
  }

  const STR_TITLE = "We Just Sent You an Email";
  const STR_SUBTITLE = "Open the email and click on the Verification button";
  const STR_SUBSUBTITLE = "It may have been filtered into the promotion or spam folders";
  const STR_SENDAGAIN = "I didn’t receive an email, please send again";
  const STR_CHANGE = "Change email account";

export const SignUpFour = (props?:LoginArgs) => {

    const {} = props || DEFAULT_ARGS;


    return `
    <signup-full-wide title="${STR_TITLE}">
        
        <sub-title slot="subtitle" title="${STR_SUBTITLE}"></sub-title>
        <sub-title slot="subtitle" title="${STR_SUBSUBTITLE}"></sub-title>
        <spacer-fourty slot="confirmation"></spacer-fourty>
        <confirmation-button slot="confirmation"></confirmation-button>

        <plain-blue title="${STR_CHANGE}" slot="main"></plain-blue>
        <contact-email slot="contact"></contact-email>
        </signup-full-wide>

    `
}

SignUpFour.args = DEFAULT_ARGS;