import "@elements/cards/icon-banner-card";
import "@elements/cards/banner-card";
import "@elements/titles/plain-black-thick";
import "@elements/titles/plain-black";
import "@elements/images/search-card-img";
import "@elements/dividers/horizontal-full";
import "@elements/admin/templates-layout/search-card-full";
import "@elements/cards/age-group";
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

}

const DEFAULT_ARGS:CardArgs = {
  color:"blue",
  icon:"",
  label:"Anat",
  amount:16,
  number:true,
  likes:24,
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
const STR_JI = "JiTeam -"
 
export const SearchCard = ({amount, number, likes}) => {
    return `<search-card-full label="Placeholder" color="blue" size="medium" fontweight="">
        <search-image slot="image" image="${STR_SEASONS}"></search-image>
        <plain-black-thick slot="title" title="${STR_GEMATRIA}"></plain-black-thick>
        <plain-black slot="subtitle" title="${STR_PLAYED}" ${number && "number"} amount="${amount}"></plain-black>
        <horizontal-full slot="subtitle"></horizontal-full>
        <plain-black slot="subtitle" title="${STR_LIKED}" ${number && "number"} amount="${likes}"></plain-black>
        <age-group slot="age" icon="${STR_AGEICON}" label="${STR_ALL}"></age-group>
        <age-group slot="language" icon="${STR_WORLDICON}" label="${STR_ENGLISH}"></age-group>
        <banner-card slot="banner" icon="${STR_ICON}" label="${STR_ANAT}" team="${STR_JI}"></banner-card>

    </search-card-full>
    `
}




SearchCard.args = DEFAULT_ARGS;
SearchCard.argTypes = {
  color: {
    control: {
      type: 'inline-radio',
      options: ["blue", "green", "white"]
    }
  }
}