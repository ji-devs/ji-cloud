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

export const Search = (props?: SearchArgs) => {

    const { checked } = props || DEFAULT_ARGS;

    return `
    <search-bar>
    <search-dropdown slot="dropdown" placeholder="All ages">
        
        <searchbox-li-check ${checked && 'checked'}>Passover</searchbox-li-check>
    
    </search-dropdown>
    <search-dropdown slot="dropdown-language" placeholder="All languages"></search-dropdown>
    <button-rect slot="button" iconBefore="magnifyer" size="medium" bold=true color="${STR_RED}" largetext=true>Search</button-rect>
    <title-ji slot="advanced" color="lightblue" weight="x-bold">Advanced <br> Search</title-ji>

    </search-bar>
    `
}

Search.args = DEFAULT_ARGS;
