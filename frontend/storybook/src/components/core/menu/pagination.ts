import {argsToAttrs} from "@utils/attributes";
import "@elements/core/menu/search-pagination";
export default {
    title: "Core/Menu"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Pagination = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<search-pagination ${argsToAttrs(props)}></search-pagination>`;
}

Pagination.args = DEFAULT_ARGS;