import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/buttons/rectangle";
import {Color, Size, IconAfter, IconBefore} from "@elements/core/buttons/rectangle";

export default {
  title: 'Core / Buttons',
}

interface Args {
  contents: string,
  color: Color,
  size: Size,
  bold: boolean,
  italic: boolean,
  iconAfter:IconAfter | "none",
  iconBefore:IconAfter | "none",
}

const DEFAULT_ARGS:Args = {
  contents: "Submit",
  color: "red",
  size: "medium",
  bold: false,
  italic: false,
  iconAfter: "none",
  iconBefore: "none",
}

export const Rectangle = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {contents, ...buttonProps} = props;

    return `<button-rect ${argsToAttrs(deleteNone(buttonProps))}>${contents}</button-rect>`
}

//Continuing the previous example
Rectangle.argTypes = {
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
  iconBefore: {
    control: {
      type: 'inline-radio',
      options: ["none"]
    }
  },
  iconAfter: {
    control: {
      type: 'inline-radio',
      options: ["none", "arrow"]
    }
  }
}


Rectangle.args = DEFAULT_ARGS;
