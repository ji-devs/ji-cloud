import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/lists/list-horizontal";
import "@elements/core/lists/li-check";
export default {
    title: "Core / Lists"
}

interface Args {
    contents: string,
    selected: boolean,
    width: number,
}

const DEFAULT_ARGS:Args = {
    contents: "hello",
    selected: true ,
    width: 300
}

export const LiCheck = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {width,contents, ...listProps} = props
    return `
        <ul style="width: ${width}px">
            <li-check ${argsToAttrs(props)}>${contents}</li-check>
        </ul>`;
}

LiCheck.args = DEFAULT_ARGS;