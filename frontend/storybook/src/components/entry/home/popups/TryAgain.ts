import "@elements/core/popups/popup-container";
import {Color,Size} from "@elements/core/popups/popup-container";
import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/titles/variants/title-section";
import "@elements/entry/home/widgets/studentcode-section";
import "@elements/core/dividers/square-divider";



const STR_TITLE="Enter your student code";

 

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
  
  
 


export const TryAgain= (props?:Partial<Args>) => {
const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <popup-container ${argsToAttrs(deleteNone(popupProps))}>
 <studentcode-section kindimage="tryagain">
 <title-section titlecolor="darkblue" title="${STR_TITLE}" size="small" slot="title"></title-section>
 <square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>


</studentcode-section>




 </popup-container>


    `
}

TryAgain.args = DEFAULT_ARGS;
TryAgain.argTypes = {
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







