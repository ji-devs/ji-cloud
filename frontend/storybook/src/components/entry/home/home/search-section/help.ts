import "@elements/entry/home/home/search-section/search-section-help";
export default {
    title: "Entry / Home / Home / Search section",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Help = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <home-search-section-help slot="help"></home-search-section-help>
    `;
};

Help.args = DEFAULT_ARGS;
