import "@elements/entry/home/search-section/search-bar";
import "@elements/entry/home/search-section/search-section-select";

export default {
    title: 'Entry / Home / Search section',
}

interface SearchArgs {

}

const DEFAULT_ARGS: SearchArgs = {

}

export const SearchBar = (props?: SearchArgs) => {

    return `
        <div style="padding:30px;background:#00800066;">
            <home-search-bar slot="search-bar">

                <input slot="query" placeholder="What are you looking for?">
                <home-search-section-select slot="age" value="All ages"></home-search-section-select>
                <home-search-section-select slot="language" value="All languages"></home-search-section-select>
                <button-rect slot="button" bold color="red">Search</button-rect>
                <button-text slot="advanced" color="white" weight="bold">Search <br> Advanced</button-text>

            </home-search-bar>
        </div>
    `
}

SearchBar.args = DEFAULT_ARGS;
