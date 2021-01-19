import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/text";
import {Color, Size, Weight} from "@elements/core/buttons/text";
export default {
  title: 'Core / Buttons',
}

interface Args {
  contents: string,
  color: Color,
  size: Size,
  weight: Weight,
  italic: boolean,
}

const DEFAULT_ARGS:Args = {
  contents: "Submit",
  color: "blue",
  size: "medium",
  weight: "normal",
  italic: false,
}

export const Text = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {contents, ...buttonProps} = props;
    return `<button-text ${argsToAttrs(buttonProps)}>${contents}</button-text>`
}

//Continuing the previous example
Text.argTypes = {
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
  },
  weight: {
    control: {
      type: 'inline-radio',
      options: ["normal", "medium", "bold"]
    }
  }
}


Text.args = DEFAULT_ARGS;
