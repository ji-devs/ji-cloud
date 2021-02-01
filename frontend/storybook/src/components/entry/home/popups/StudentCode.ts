import "@elements/core/popups/popup-container";
import {Color,Size} from "@elements/core/popups/popup-container";
import "@elements/entry/home/widgets/studentcode-section";
import { argsToAttrs, deleteNone } from "@utils/attributes";



  


export default {
  title: 'Entry /Home/Widgets/Popups',
  }

  interface Args{
    color: Color,
    size: Size,
   }
  
  const DEFAULT_ARGS:Args = {
    color:"green",
    size: "large",
   }
  
  


export const StudentCode= (props?:Partial<Args>) => {
  const {...popupProps} = props;
  props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <popup-container ${argsToAttrs(deleteNone(popupProps))}>
 <studentcode-section kindimage="play">
 
 </studentcode-section>




 </popup-container>


    `
}

StudentCode.args = DEFAULT_ARGS;
StudentCode.argTypes = {
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






