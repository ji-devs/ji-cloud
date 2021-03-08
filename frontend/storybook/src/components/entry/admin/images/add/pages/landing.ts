import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/images/add/pages/landing";
import {ImageKind} from "@elements/entry/admin/images/add/pages/landing";
import "@elements/entry/admin/images/add/buttons/add";

export default {
    title: 'Entry/Admin/Images/Add',
}


interface Args {
    imageKind: ImageKind
}

const DEFAULT_ARGS:Args = {
    imageKind: "sticker"
}

export const Landing = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
  
  return `
  <image-add-page ${argsToAttrs(props)}>
  <button-add slot="button"></button-add>

    </image-add-page>
    `
}

Landing.args = DEFAULT_ARGS;

Landing.argTypes = {
  imageKind: {
    control: {
      type: 'inline-radio',
      options: ["sticker", "canvas"]
    }
  },
}
