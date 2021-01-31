 import "@elements/entry/home/sections/aboutus-section";
import "@elements/core/titles/variants/title-section";
import {Color, FontSize} from "@elements/core/titles/title-sub-paragraph";
import {MovingCard} from "~/components/core/cards/card";
 
export default {
  title: 'Homepage',
}

interface Args{
  color: Color,
  
}

const DEFAULT_ARGS:Args = {
  color:"yellow",
  
}

 
  
  const PATH_SARAHN="sarah-nazirah.png";
const STR_TITLEPARENTS = "Parents";
const STR_SUBTITLEPARENTS = "Sarah Nazirah, Mexico";
const STR_BODYPARENT="I want to tell you, because of JI, my children are learning Hebrew and English simultaneously. For my children, you are not only teaching two children, you are also saving their souls. I reaffirm that they have achieved educational rehabilitation, thanks to JI.";


const PATH_SARAHH="Sara-Halberstadt.png";
const STR_TITLETEACHERS = "Teachers";
const STR_SUBTITLETEACHERS = "Sarah Nazirah, Mexico";
const STR_BODYTEACHER="I want to tell you, because of JI, my children are learning Hebrew and English simultaneously. For my children, you are not only teaching two children, you are also saving their souls. I reaffirm that they have achieved educational rehabilitation, thanks to JI.";



  const  ITEMSTeacher=[
    {imgPath:PATH_SARAHH,
    title:STR_TITLETEACHERS,
    subtitle:STR_SUBTITLETEACHERS,
    body: STR_BODYTEACHER
    
  }]
  const  activeIndexTeacher=0;


  const  ITEMSPARENTS=[
    {imgPath:PATH_SARAHN,
    title:STR_TITLEPARENTS,
    subtitle:STR_SUBTITLEPARENTS,
    body: STR_BODYPARENT
  }]




   export const AboutUsParagraph = (props?:Partial<Args>) => {
  const {...titleProps} = props;
  props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `

    <aboutus-section>
 <div slot="cardteachers">${MovingCard({items:ITEMSTeacher,activeIndex:activeIndexTeacher})}</div>
 <div slot="cardparents">${MovingCard({items:ITEMSPARENTS,activeIndex:activeIndexTeacher})}</div>
    </aboutus-section>
    `
}
AboutUsParagraph.args = DEFAULT_ARGS;
AboutUsParagraph.argTypes = {
  color: {
      control: {
          type: 'inline-radio',
          options: ["white", "black", "yellow"]
      }
  } 
  
 
}







