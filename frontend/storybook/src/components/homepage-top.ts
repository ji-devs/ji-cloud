import "@elements/entry/home/sections/header-blue";
import "@elements/core/titles/ji";
import { Search } from "~/components/homepage-search";
export default {
  title: 'Homepage',
}

export const TopSection = () => {
    return `
    <header-blue>
        <title-ji slot="subtitle" size="x-large" color="lightblue" weight="x-bold">Learning Through</title-ji>
        <title-ji slot="subtitle" size="x-large" color="lightgreen" weight="x-bold"> Creation</title-ji>
        <div slot="search">
        ${Search()}
        </div>
      
        <title-ji slot="undertext" size="large" color="black" weight="thin">Make learning awesome with</title-ji>
        <title-ji slot="undertext" size="large" color="black" weight="bold">10,345</title-ji>
        <title-ji slot="undertext" size="large" color="black" weight="thin">JIGs</title-ji>
       
        
    </header-blue>
    `
}



