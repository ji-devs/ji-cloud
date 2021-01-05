import "@elements/admin/templates-layout/signup-full-wide";
import "@elements/titles/subtitle";
import "@elements/titles/plain-blue";



export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
    title:string,
    subtitle:string,
    subtitletwo:string,
    title_two:string,
    title_three:string,
  }

  const DEFAULT_ARGS:LoginArgs = {
    title: "We just sent you an email",
    subtitle:"Open the email and click on the Verification button",
    subtitletwo: "It might had got to the promotion or spam’s folders",
    title_two:"I didn’t receive an email, Please send again",
    title_three:"Change email account"

   
  }

export const SignUpFour = (props?:LoginArgs) => {

    const {title,subtitle,subtitletwo, title_two, title_three} = props || DEFAULT_ARGS;


    return `
    <signup-full-wide title="${title}">
        
        <sub-title slot="subtitle" title="${subtitle}"></sub-title>
        <sub-title slot="subtitle" title="${subtitletwo}"></sub-title>
        <plain-blue title="${title_two}" slot="main"></plain-blue>
        <plain-blue title="${title_three}" slot="main"></plain-blue>
        </signup-full-wide>

    `
}

SignUpFour.args = DEFAULT_ARGS;