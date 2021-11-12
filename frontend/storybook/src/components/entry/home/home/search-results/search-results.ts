import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/home/home/search-results/search-results";
import { ResultsSection } from "./search-results-section";

export default {
    title: "Entry / Home / Home / Search results",
};

interface Args {
    resultsCount: number;
    query: string;
}

const DEFAULT_ARGS: Args = {
    resultsCount: 1,
    query: "Hebrew",
};

export const SearchResults = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <home-search-results ${argsToAttrs(props)}>
            ${ResultsSection()}
        </home-search-results>
    `;
};

SearchResults.args = DEFAULT_ARGS;
