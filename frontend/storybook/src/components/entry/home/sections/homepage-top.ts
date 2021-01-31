import "@elements/entry/home/sections/header-blue";
import "@elements/core/titles/ji";
import { Search } from "~/components/entry/home/sections/homepage-search";
export default {
  title: 'Entry/ Home / Section',
}

const STR_LEARNING = "Learning Through";
const STR_CREATION = "Creation";
const STR_MAKELEARNING ="Make learning awesome with ";
const STR_NUMBER = "10,345 ";
const STR_JIG = "JIGs";


export const TopSection = () => {
    return `
    <header-blue>
        <title-ji slot="subtitle" size="x-large" color="lightblue" weight="x-bold">${STR_LEARNING }&nbsp;</title-ji>
        <title-ji slot="subtitle" size="x-large" color="lightgreen" weight="x-bold"> ${STR_CREATION}</title-ji>
        <div slot="search">
        ${Search()}
        </div>
      
        <title-ji slot="undertext" size="large" color="black" weight="thin">${STR_MAKELEARNING} &nbsp; </title-ji>
        <title-ji slot="undertext" size="large" color="black" weight="bold">${STR_NUMBER} &nbsp;</title-ji>
        <title-ji slot="undertext" size="large" color="black" weight="thin">${STR_JIG}</title-ji>
       
        
    </header-blue>
    `
}



