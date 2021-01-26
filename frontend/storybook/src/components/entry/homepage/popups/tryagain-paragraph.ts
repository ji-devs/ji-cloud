import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";
import "@elements/core/popups/popup-container";
import {Color,Size} from "@elements/core/popups/popup-container";
import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/titles/variants/title-section";
import "@elements/entry/home/widgets/studentcode-section";
import "@elements/core/dividers/square-divider";


const STR_SMALL="small";
const STR_DARKBLUE="darkblue";
const STR_TITLE="Enter your student code";
const STR_PATH="Illustration_JIG_Sad_1.png";
const STR_PATHBALOON="Baloon_1@2x.png";
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
  
  
 


export const tryAgain= (props?:Partial<Args>) => {
const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <popup-container ${argsToAttrs(deleteNone(popupProps))}>
 <studentcode-section>
 <title-section titlecolor="darkblue" title="${STR_TITLE}" size="small" slot="title"></title-section>
 <square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>


</studentcode-section>




 </popup-container>


    `
}

tryAgain.args = DEFAULT_ARGS;








