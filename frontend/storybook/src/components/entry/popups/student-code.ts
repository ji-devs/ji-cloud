import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";
import "@elements/core/popups/popup";
import {Color,Size} from "@elements/core/popups/popup";
import {argsToAttrs, deleteNone} from "@utils/attributes";


 
export default {
    title: 'Popups',
  }

  interface Args{
    color: Color,
    size: Size,
  }
  
  const DEFAULT_ARGS:Args = {
    color:"peach",
    size: "medium",
  }
  
  
  const STR_TITLE="Help";
 


export const studentCode= (props?:Partial<Args>) => {
const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <template-popups  ${argsToAttrs(deleteNone(popupProps))}>
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







