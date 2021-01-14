import "@elements/cards/icon-banner-card";
import "@elements/cards/blue-card";

export default {
  title: 'Cards',
}

interface CardArgs {
 color:string,

}

const DEFAULT_ARGS:CardArgs = {
  color:"blue",
}

export const IconBannerCard = () => {
    return `<icon-banner-card label="Placeholder" color="blue" size="medium" fontweight=""/>`
}

export const BannerCard = (props?:CardArgs) => {

  const {color,} = props || DEFAULT_ARGS;
  return `<banner-card label="Placeholder" path="" color="${color}"/>`
}

export const BlueCard = () => {
  return `<blue-card color=""/>`
}


BannerCard.args = DEFAULT_ARGS;