import "@elements/entry/home/home/search-section/search-section";
import { Mode } from "@elements/entry/home/home/search-section/search-section";
import "@elements/entry/home/home/search-section/search-section-help";
import { argsToAttrs } from "@utils/attributes";
import { SearchBar } from "~/components/entry/home/home/search-section/search-bar";
export default {
    title: "Entry / Home / Home / Search section",
};

interface Args {
    mode: Mode;
    resultsCount: number;
}

const DEFAULT_ARGS: Args = {
    mode: "home",
    resultsCount: 3234,
};

export const SearchSection = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <home-search-section ${argsToAttrs(props)}>
            ${SearchBar()}
            <home-search-section-help slot="help"></home-search-section-help>
        </home-search-section>
    `;
};

SearchSection.args = DEFAULT_ARGS;
SearchSection.argTypes = {
    mode: {
        control: {
            type: "inline-radio",
            options: ["home", "results"],
        },
    },
};
