import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/search/search-full";
import "@elements/entry/search/search-header";
import { Search} from "~/components/entry/home/sections/homepage-search";

export default {
    title: "Entry/Home/Search"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const SearchHeader = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<search-header ${argsToAttrs(props)}>
    <div slot="header">    
        ${Search()}
        </div>
    
    <search-header/>`;
}

SearchHeader.args = DEFAULT_ARGS;