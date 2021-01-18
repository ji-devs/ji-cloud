import "@elements/cards/icon-banner-card";
import "@elements/cards/banner-card";
import "@elements/cards/blue-card";
import {Color} from "@elements/cards/banner-card";
export default {
  title: 'Cards',
}

interface CardArgs {
 color:Color,
 icon:string,
 label:string,
 imghidden:boolean,
 teamhidden:boolean,
 team:string,
 

}

const DEFAULT_ARGS:CardArgs = {
  color:"blue",
  icon:"",
  label:"Anat",
  imghidden: true,
  teamhidden:true,
  team:"Ji Team - ",
  
}

export const IconBannerCard = () => {
    return `<icon-banner-card label="Placeholder" color="blue" size="medium" fontweight=""/>`
}

export const BannerCard = (props?:CardArgs) => {

  const {color,label, icon, imghidden, team, teamhidden} = props || DEFAULT_ARGS;
  return `<banner-card label="${label}" icon="${icon}" color="${color}" ${imghidden && "imghidden"} ${teamhidden && "teamhidden"} team="${team}" />`
}

export const BlueCard = () => {
  return `<blue-card color=""/>`
}


BannerCard.args = DEFAULT_ARGS;
BannerCard.argTypes = {
  color: {
    control: {
      type: 'inline-radio',
      options: ["blue", "green", "white"]
    }
  }
}