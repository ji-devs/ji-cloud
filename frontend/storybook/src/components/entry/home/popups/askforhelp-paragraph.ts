import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";
import "@elements/core/popups/popup-container";
import {Color,Size} from "@elements/core/popups/popup-container";
import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/titles/variants/title-section";
import "@elements/entry/home/widgets/studentcode-section";
 import "@elements/core/dividers/square-divider";
 import {ColorBorder} from "@elements/core/dividers/square-divider";
 import {Size as DividerSize} from "@elements/core/dividers/square-divider";
const STR_SMALL="small";
const STR_DARKBLUE="darkblue";
const STR_TITLE="Enter your student code";

const STR_BLUE="blue";
 

export default {
    title: 'Entry /Home/Widgets/Popups',
  }

  interface Args{
    color: Color,
    size: Size,
    colorborder:ColorBorder,
    
  }
  
  const DEFAULT_ARGS:Args = {
    color:"green",
    size: "large",
    colorborder: "blue"
  }
  
  
 


export const askForHelp= (props?:Partial<Args>) => {
const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <popup-container  ${argsToAttrs(deleteNone(popupProps))}>
 <studentcode-section>
 <title-section titlecolor="darkblue" title="${STR_TITLE}" size="small" slot="title"></title-section>

<square-divider colorborder="small" size="blue" slot="square"></square-divider>
<square-divider colorborder="small" size="blue" slot="square"></square-divider>
<square-divider colorborder="small" size="blue" slot="square"></square-divider>
<square-divider colorborder="small" size="blue" slot="square"></square-divider>




</popup-container>




 </popup-container>


    `
}

askForHelp.args = DEFAULT_ARGS;
askForHelp.argTypes = {
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
image: {
    control: {
        type: 'inline-radio',
        options: ["Illustration_JIG_Sad_1.png.png", "Illustration_JIG_Sad_1.png.png", "Illustration_JIG_Sad_1.png.png"]
    }
}
}







