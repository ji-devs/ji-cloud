 import "@elements/entry/home/sections/logout-section";
import "@elements/core/titles/variants/title-section";
  import "@elements/core/popups/popup-container";
  import {Color,Size} from "@elements/core/popups/popup-container";
import { argsToAttrs, deleteNone } from "@utils/attributes";
import "@elements/core/buttons/rectangle" ;
import "@elements/core/buttons/text";

export default {
  title: 'Entry /Home/Widgets/Popups',
}

interface Args{
  color: Color,
  size: Size,
}

const DEFAULT_ARGS:Args = {
  color:"peach",
  size: "medium",
}

 
const STR_LOGOUT="Logout";
const STR_Cancel="Cancel";


export const Logout = (props?:Partial<Args>) => {
  const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
<popup-container   ${argsToAttrs(deleteNone(popupProps))}>

<logout-section >
     
<button-rect size="large" color="blue" slot="button">${STR_LOGOUT}</button-rect>
<button-text  size="large"  color="blue" slot="textbutton" >${STR_Cancel}</button-text>
</logout-section>

</popup-container>
    `
}

Logout.args = DEFAULT_ARGS;
Logout.argTypes = {
  color: {
      control: {
          type: 'inline-radio',
          options: ["peach", "green"]
      }
  },
  size: {
    control: {
        type: 'inline-radio',
        options: ["medium", "large"]
    }
},

}