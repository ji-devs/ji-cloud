import {argsToAttrs, argToAttr} from "@utils/attributes";
import "@elements/core/lists/li-check-collection";
import "@elements/core/lists/li-check";

export default {
    title: "Core / Lists"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const LiCheckCollection = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <li-check-collection label="hello0">
            <li-check>Hello1</li-check>
            <li-check>Hello2</li-check>
            <li-check>Hello3</li-check>
            <li-check>Hello4</li-check>
        </li-check-collection>
    `;
}

LiCheckCollection.args = DEFAULT_ARGS;



interface Args2 {
    open: boolean,
}
const DEFAULT_ARGS2:Args2 = {
    open: false,
}
export const LiCheckCollectionInSelect = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <div style="width:400px;">
            <dropdown-select ${argsToAttrs(props)}>
                <li-check-collection label="hello0" open>
                    <li-check>Hello1</li-check>
                    <li-check>Hello2</li-check>
                    <li-check>Hello3</li-check>
                    <li-check>Hello4</li-check>
                </li-check-collection>
                <li-check>Hello1</li-check>
                <li-check>Hello2</li-check>
                <li-check>Hello3</li-check>
                <li-check>Hello4</li-check>
                <li-check>Hello1</li-check>
                <li-check>Hello2</li-check>
                <li-check>Hello3</li-check>
                <li-check>Hello4</li-check>
                <li-check>Hello1</li-check>
                <li-check>Hello2</li-check>
                <li-check>Hello3</li-check>
                <li-check>Hello4</li-check>
            </dropdown-select>
        </div>
    `;
}

LiCheckCollectionInSelect.args = DEFAULT_ARGS2;
