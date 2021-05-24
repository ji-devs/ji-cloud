import "@elements/entry/home/search-section/search-bar";

export default {
    title: 'Entry / Home / Search section',
}

interface SearchArgs {

}

const DEFAULT_ARGS: SearchArgs = {

}

export const SearchBar = (props?: SearchArgs) => {

    return `
        <home-search-bar slot="search-bar">

            <input slot="query" placeholder="What are you looking for?">
            <div slot="age">All ages</div>
            <div slot="language">All languages</div>
            <button-rect slot="button" bold color="red">Search</button-rect>
            <button-text slot="advanced" color="white" weight="bold">Search <br> Advanced</button-text>

        </home-search-bar>
    `
}

SearchBar.args = DEFAULT_ARGS;
