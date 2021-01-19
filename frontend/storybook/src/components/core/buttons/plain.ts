import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/plain";
import {Color, Size} from "@elements/core/buttons/plain";
export default {
  title: 'Core / Buttons',
}

interface Args {
  contents: string,
  color: Color,
  size: Size,
  bold: boolean,
  italic: boolean,
}

const DEFAULT_ARGS:Args = {
  contents: "Submit",
  color: "red",
  size: "medium",
  bold: false,
  italic: false,
}

export const Plain = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {contents, ...buttonProps} = props;
    return `<button-plain ${argsToAttrs(buttonProps)}>${contents}</button-plain>`
}

//Continuing the previous example
Plain.argTypes = {
  color: {
    control: {
      type: 'inline-radio',
      options: ["red", "blue", "green", "white"]
    }
  },
  size: {
    control: {
      type: 'inline-radio',
      options: ["small", "medium", "large"]
    }
  }
}


Plain.args = DEFAULT_ARGS;
