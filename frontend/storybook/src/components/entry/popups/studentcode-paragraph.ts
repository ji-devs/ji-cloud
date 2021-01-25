import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";
import "@elements/core/popups/popup";
import {Color,Size} from "@elements/core/popups/popup";
import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/titles/variants/title-section";
import "@elements/entry/popup/sections/studentcode-section";
import "@elements/core/dividers/square-divider";


const STR_SMALL="small";
const STR_DARKBLUE="darkblue";
const STR_TITLE="Enter your student code";
const STR_PATH="Image_JIG_StudentCode@2x.png";
const STR_BLUE="blue";
 


export default {
    title: 'Popups',
  }

  interface Args{
    color: Color,
    size: Size,
  }
  
  const DEFAULT_ARGS:Args = {
    color:"green",
    size: "large",
  }
  
  
 


export const studentCode= (props?:Partial<Args>) => {
const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <template-popups  ${argsToAttrs(deleteNone(popupProps))}>
 <studentcode-section>
 <title-section titlecolor="${STR_DARKBLUE}" title="${STR_TITLE}" size="${STR_SMALL}" slot="title"></title-section>
 <square-divider colorborder="${STR_SMALL}" size="${STR_BLUE}" slot="square"></square-divider>
<square-divider colorborder="${STR_SMALL}" size="${STR_BLUE}" slot="square"></square-divider>
<square-divider colorborder="${STR_SMALL}" size="${STR_BLUE}" slot="square"></square-divider>
<square-divider colorborder="${STR_SMALL}" size="${STR_BLUE}" slot="square"></square-divider>
 <img-ui path="${STR_PATH}" slot="leftimg"></img-ui>
</studentcode-section>




 </template-popups>


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
}
}







