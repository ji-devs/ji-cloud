import "@elements/entry/home/sections/search-bar";
import "@elements/entry/home/search-dropdown"; 
import "@elements/core/buttons/rectangle";
import "@elements/core/lists/searchbox-li-check";
import "@elements/core/titles/ji";
export default {
    title: 'Homepage',
}

interface SearchArgs {
    checked: boolean

}

const DEFAULT_ARGS: SearchArgs = {
    checked: false

}

const STR_RED = "red";
const STR_SEARCH = "Search";
const STR_LANGUAGES = "All languages";
const STR_ADVANCED = "Advanced ";



export const Search = (props?: SearchArgs) => {

    const { checked } = props || DEFAULT_ARGS;

    return `
    <search-bar>
    <search-dropdown slot="dropdown" placeholder="All ages">
        
        <searchbox-li-check ${checked && 'checked'}>Passover</searchbox-li-check>
    
    </search-dropdown>
    <search-dropdown slot="dropdown-language" placeholder="${STR_LANGUAGES}"></search-dropdown>
    <button-rect slot="button" iconBefore="magnifyer" size="x-large" bold=true color="${STR_RED}" largetext=true>${STR_SEARCH}</button-rect>
    <title-ji slot="advanced" color="lightblue" weight="x-bold" link=true>${STR_ADVANCED} <br> ${STR_SEARCH}</title-ji>

    </search-bar>
    `
}

Search.args = DEFAULT_ARGS;
