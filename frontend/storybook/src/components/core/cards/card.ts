 
  import "@elements/entry/home/about-us/moving-card";
  import {Item} from "@elements/entry/home/about-us/moving-card";

interface Args{
items:Array<Item>,
activeIndex:number,
}

const DEFAULT_ARGS:Args={
  activeIndex:0,
items:[
  {imgPath:"Sara-Halberstadt.png",
  title:"Teachers",
  subtitle:"Sarah Nazirah, Mexico",
  body:"I want to tell you, because of JI, my children are learning Hebrew and English simultaneously. For my children, you are not only teaching two children, you are also saving their souls. I reaffirm that they have achieved educational rehabilitation, thanks to JI."
  
}],

}


export default {
    title: 'Homepage',
  }

    export const MovingCard= (props?:Partial<Args>) => {
      props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
      const {items,activeIndex} = props;

      const nItems=items.length;


      const json=JSON.stringify(items);
 
       console.log("nItems"  ,nItems );


    return `
      <moving-card items='${json}' nItems="${nItems}"/>

 

     
    `
}
MovingCard.args = DEFAULT_ARGS;
