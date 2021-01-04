import "@elements/buttons/google";
export default {
  title: 'Google Button',
}

interface GoogleArgs {
    label: string,
    path: string,
   
  
  }
  
  const DEFAULT_ARGS:GoogleArgs = {
    label: "Sign in with Google",
    path:"/icn-google-button-64.svg"
  }

export const GoogleButton = (props?:GoogleArgs) => {
    const {label, path} = props || DEFAULT_ARGS;

    return `<google-button label="${label}" path="${path}"/>`
}




GoogleButton.args = DEFAULT_ARGS
