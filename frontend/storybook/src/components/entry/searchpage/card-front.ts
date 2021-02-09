import "@elements/core/cards/parts/image";
import "@elements/core/dividers/horizontal-full";
import "@elements/entry/search/card-front";
import "@elements/widgets/tags/icon"; 
import "@elements/core/cards/parts/dropdown";
import "@elements/core/pills/pill-close";
import "@elements/core/titles/ji";
import "@elements/core/cards/parts/banner";
import { Kind } from "@elements/entry/search/card-front";

import {Color} from "@elements/core/cards/parts/banner";
export default {
  title: 'Entry / Home / Search',
}

interface CardArgs {
 color:Color,
 icon:string,
 label:string,
 amount:number,
 number:boolean,
 likes: number,
 imghidden:boolean,
 teamhidden:boolean,
 team:string,
 type:Kind,
 bannertext:string,
 bannernumber:number,
 

}

const DEFAULT_ARGS:CardArgs = {
  color:"blue",
  icon:"",
  label:"Anat",
  amount:16,
  number:true,
  likes:24,
  imghidden: true,
  teamhidden:true,
  team:"Ji Team - ",
  type:"",
  bannertext:"JIGs",
  bannernumber: 16
  
}

const STR_SEASONS = "seasons.jpg";
const STR_GEMATRIA = "The Big Gematria challenge";
const STR_PLAYEDAMOUNT = "24";
const STR_ALL ="All ages";
const STR_ENGLISH = "English";
const STR_ANAT = "Anat";
const STR_LIKEDAMOUNT = "16";




 
export const SearchCardFront = (props?:Partial<CardArgs>) => {

    const {type} = props || DEFAULT_ARGS;

    return `<card-front label="Placeholder" color="blue" size="medium" fontweight="" type=${type}>
        <search-image slot="image" image="${STR_SEASONS}"></search-image>
        <title-ji color="black" slot="title">${STR_GEMATRIA}</title-ji>
        <title-ji color="black" weight="bold" slot="played">${STR_PLAYEDAMOUNT}</title-ji>
        <horizontal-full slot="subtitle"></horizontal-full>
        <title-ji color="black" slot="liked"  weight="bold">${STR_LIKEDAMOUNT}</title-ji>

        <tag-icon kind="age" slot="age" label="${STR_ALL}"></tag-icon>
        <tag-icon kind="lang" slot="language" label="${STR_ENGLISH}"></tag-icon>
        <card-banner slot="banner" label="${STR_ANAT}" color="blue"></card-banner>

    </card-front>
    `
}



   SearchCardFront.args = DEFAULT_ARGS;
