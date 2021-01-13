import "@elements/cards/icon-banner-card";
import "@elements/cards/blue-card";

export default {
  title: 'Cards',
}

export const IconBannerCard = () => {
    return `<icon-banner-card label="Placeholder" color="blue" size="medium" fontweight=""/>`
}

export const BannerCard = () => {
  return `<banner-card label="Placeholder" path="" color=""/>`
}

export const BlueCard = () => {
  return `<blue-card color=""/>`
}


