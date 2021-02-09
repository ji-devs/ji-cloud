import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/search/search-section";

export default {
    title: "Entry/Home/Search/Section"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const SearchSection = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<search-section ${argsToAttrs(props)}>
    
    </search-section>`;
}

SearchSection.args = DEFAULT_ARGS;