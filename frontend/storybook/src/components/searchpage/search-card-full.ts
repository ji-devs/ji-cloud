import "@elements/cards/icon-banner-card";
import "@elements/cards/banner-card";
import "@elements/titles/plain-black-thick";
import "@elements/titles/plain-black";
import "@elements/titles/plain-white";
import "@elements/images/search-card-img";
import "@elements/dividers/horizontal-full";
import "@elements/admin/templates-layout/search-card-full";
import "@elements/cards/age-group";
import "@elements/dropdowns/card-dropdown";
import "@elements/lists/pill";
import {Color} from "@elements/cards/banner-card";
export default {
  title: 'Search Page',
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
  
}

const STR_SEASONS = "seasons.jpg";
const STR_GEMATRIA = "The Big Gematria challenge";
const STR_PLAYED = "Played";
const STR_LIKED = "Liked";
const STR_AGEICON = "Icn_Age.svg";
const STR_ALL ="All ages";
const STR_WORLDICON ="icn-language.svg";
const STR_ENGLISH = "English";
const STR_ICON = "";
const STR_ANAT = "Anat";
const STR_JI = "JiTeam -";
const STR_BEREISHIT = "Bereishit Stories, Briat Haolam Story, Adam veChava"
const STR_LIGHTBLUE = "lightblue";
const STR_DARKGREY = "darkgrey";
const STR_TWOWEEKS = "2 weeks ago"



 
export const SearchCard = (props?:CardArgs) => {

    const {number, likes, amount, color, imghidden, teamhidden, team} = props || DEFAULT_ARGS;

    return `<search-card-full label="Placeholder" color="blue" size="medium" fontweight="" front>
        <search-image slot="image" image="${STR_SEASONS}"></search-image>
        <plain-black-thick slot="title" title="${STR_GEMATRIA}"></plain-black-thick>
        <plain-black slot="subtitle" title="${STR_PLAYED}" ${number && "number"} amount="${amount}"></plain-black>
        <horizontal-full slot="subtitle"></horizontal-full>
        <plain-black slot="subtitle" title="${STR_LIKED}" ${number && "number"} amount="${likes}"></plain-black>
        <age-group slot="age" icon="${STR_AGEICON}" label="${STR_ALL}" color="${STR_DARKGREY}"></age-group>
        <age-group slot="language" icon="${STR_WORLDICON}" label="${STR_ENGLISH}" color="${STR_DARKGREY}"></age-group>
        <banner-card slot="banner" icon="${STR_ICON}" label="${STR_ANAT}" team="${STR_JI}" color="${color}" ${imghidden && "imghidden"} ${teamhidden && "teamhidden"} team="${team}"></banner-card>

    </search-card-full>
    `
}


interface CardBackArgs {
    negative: boolean
    
   
   }
   
   const DEFAULT_BACK_ARGS:CardBackArgs = {
    negative:true,
     
   }

   const STR_HEBREW = "Hebrew letters"



export const SearchCardBack = (props?:CardBackArgs) => {

    const {negative} = props || DEFAULT_BACK_ARGS;

    return `<search-card-full label="Placeholder" color="blue" size="medium" fontweight="" back>
       <plain-white title="${STR_BEREISHIT}" bold="true" slot="title"></plain-white>
       <age-group slot="subtitle" icon="${STR_WORLDICON}" label="${STR_TWOWEEKS}" color="${STR_LIGHTBLUE}"></age-group>
       <card-dropdown slot="dropdowns">
            <pill-listitem label=${STR_HEBREW } ${negative && 'negative'} slot="title"></pill-listitem>
       </card-dropdown>

    </search-card-full>
    `
}




SearchCardBack.args = DEFAULT_BACK_ARGS;
SearchCard.args = DEFAULT_ARGS;
SearchCard.argTypes = {
  color: {
    control: {
      type: 'inline-radio',
      options: ["blue", "green", "white"]
    }
  }
}