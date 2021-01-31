 import "@elements/core/popups/popup-container";
import {Color,Size} from "@elements/core/popups/popup-container";
import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/titles/variants/title-section";
import "@elements/entry/home/widgets/popup-section";
import {Image} from "@elements/entry/home/widgets/popup-section";
import "@elements/core/dividers/square-divider";


const STR_SMALL="small";
const STR_DARKBLUE="darkblue";
const STR_TITLE="Enter your student code";
 const STR_BLUE="blue";
 


export default {
    title: 'Popups',
  }

  interface Args{
    color: Color,
    size: Size,
    kindimage:Image
  }
  
  const DEFAULT_ARGS:Args = {
    color:"green",
    size: "large",
    kindimage:"play"
  }
  
  
 


export const studentCode= (props?:Partial<Args>) => {
const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <popup-container  ${argsToAttrs(deleteNone(popupProps))}>
 <popup-section  ${argsToAttrs(deleteNone(popupProps))}>
 <title-section titlecolor="${STR_DARKBLUE}" title="${STR_TITLE}" size="${STR_SMALL}" slot="title"></title-section>
 <square-divider colorborder="${STR_BLUE}" size="${STR_SMALL}" slot="square"></square-divider>
<square-divider colorborder="${STR_BLUE}" size="${STR_SMALL}" slot="square"></square-divider>
<square-divider colorborder="${STR_BLUE}" size="${STR_SMALL}" slot="square"></square-divider>
<square-divider colorborder="${STR_BLUE}" size="${STR_SMALL}" slot="square"></square-divider>

 </popup-section>




 </popup-container>


    `
}

studentCode.args = DEFAULT_ARGS;
studentCode.argTypes = {
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
kindimage: {
  control: {
      type: 'inline-radio',
      options: ["play", "Ask for help", "Try again"]
    }
}
}







