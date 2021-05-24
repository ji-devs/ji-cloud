import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/search-section/search-section-advanced";

export default {
    title: "Entry / Home / Search section"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const AdvancedSearch = (props?:Args) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <home-search-section-advanced>
            <dropdown-select slot="categories" label="Categories" placeholder="Select one or more from the list"></dropdown-select>
            <dropdown-select slot="affiliation" label="Affiliation" placeholder="Select one or more from the list"></dropdown-select>
            <dropdown-select slot="goal" label="Teaching Goal" placeholder="Select from the list"></dropdown-select>
            <button-rect slot="search-button" color="blue">Search</button-rect>
        </home-search-section-advanced>
    `;
}

AdvancedSearch.args = DEFAULT_ARGS;
