import "@elements/entry/home/home/search-section/search-bar";
import "@elements/entry/home/home/search-section/search-section-select";
import { AdvancedSearch } from "./search-result-advanced";

export default {
    title: 'Entry / Home / Home / Search section',
}

interface SearchArgs {

}

const DEFAULT_ARGS: SearchArgs = {

}

export const SearchBar = (props?: SearchArgs) => {

    return `
        <home-search-bar slot="search-bar">

            <input slot="query" placeholder="What are you looking for?">
            <home-search-section-select slot="age" value="All ages"></home-search-section-select>
            <home-search-section-select slot="language" value="All languages">
                <li-check>English</li-check>
                <li-check>Hebrew</li-check>
            </home-search-section-select>
            <button-rect slot="button" bold color="red">Search</button-rect>
            ${AdvancedSearch()}
        </home-search-bar>
    `
}

SearchBar.args = DEFAULT_ARGS;
