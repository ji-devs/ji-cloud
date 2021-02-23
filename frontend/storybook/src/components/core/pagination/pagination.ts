import {argsToAttrs} from "@utils/attributes";
import "@elements/core/pagination/widget";
export default {
    title: "Core/Pagination"
}

interface Args {
    total: number;
    page: number;
}

const DEFAULT_ARGS:Args = {
    total: 10,
    page: 1
}

export const Widget = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<pagination-widget ${argsToAttrs(props)}></pagination-widget>`;
}

Widget.args = DEFAULT_ARGS;
