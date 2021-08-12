import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/home/search-section/search-section-advanced";

export default {
    title: "Entry / Home / Home / Search section"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const AdvancedSearch = (props?:Args) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <home-search-section-advanced slot="advanced">
            <button-rect kind="text" slot="opener" color="white" weight="bold">Search <br> Advanced</button-rect>
            <input-select slot="categories" label="Categories" placeholder="Select one or more from the list"></input-select>
            <input-select slot="affiliation" label="Affiliation" placeholder="Select one or more from the list"></input-select>
            <input-select slot="goal" label="Teaching Goal" placeholder="Select from the list"></input-select>
            <button-rect slot="search-button" color="blue">Search</button-rect>
        </home-search-section-advanced>
    `;
}

AdvancedSearch.args = DEFAULT_ARGS;
