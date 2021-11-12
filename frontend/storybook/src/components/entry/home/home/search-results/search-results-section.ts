import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/home/home/search-results/search-results-section";
import { Kind } from "@elements/entry/home/home/search-results/search-results-section";
import { SearchResult } from "./search-result";

export default {
    title: "Entry / Home / Home / Search results",
};

interface Args {
    mode: Kind;
    resultsCount?: number;
}

const DEFAULT_ARGS: Args = {
    mode: "jigs",
    resultsCount: 390,
};

export const ResultsSection = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <home-search-results-section ${argsToAttrs(props)} slot="sections">
            <select slot="sort">
                <option>Highest rating</option>
                <option>Best match</option>
                <option>Newest first</option>
            <select>
            ${SearchResult()}
            ${SearchResult()}
            ${SearchResult()}
            ${SearchResult()}
            ${SearchResult()}
            ${SearchResult()}
            ${SearchResult()}
            ${SearchResult()}
            <button-rect slot="load-more" color="blue">Load more</button-rect>
        </home-search-results-section>
    `;
};

ResultsSection.args = DEFAULT_ARGS;
