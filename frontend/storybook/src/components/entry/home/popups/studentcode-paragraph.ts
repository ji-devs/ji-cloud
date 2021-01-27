import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";
import "@elements/core/popups/popup-container";
import {Color,Size} from "@elements/core/popups/popup-container";
import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/titles/variants/title-section";
import "@elements/entry/home/widgets/studentcode-section";
import {Image} from "@elements/entry/home/widgets/studentcode-section";
import "@elements/core/dividers/square-divider";



const STR_TITLE="Enter your student code";
 


export default {
  title: 'Entry /Home/Widgets/Popups',
  }

  interface Args{
 
  }
  
  const DEFAULT_ARGS:Args = {
   
  }
  
  
 


export const studentCode= (props?:Partial<Args>) => {
const {...popupProps} = props;
props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `
 <popup-container color="green" size="large">
 <studentcode-section>
 <title-section titlecolor="darkblue" title="${STR_TITLE}" size="small" slot="title"></title-section>
 <square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>
<square-divider colorborder="blue" size="small" slot="square"></square-divider>
<img-ui  path="Illustration_JIG_Sad_1.png" slot="img"></img-ui>
</studentcode-section>




 </popup-container>


    `
}

studentCode.args = DEFAULT_ARGS;






