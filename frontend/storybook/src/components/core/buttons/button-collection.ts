import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/button-collection/button-collection";
import "@elements/core/buttons/button-collection/button-collection-item";

export default {
  title: 'Core / Buttons',
}
interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const ButtonCollection = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <button-collection ${argsToAttrs(props)}>
            <button-collection-item>H1</button-collection-item>
            <button-collection-item>H2</button-collection-item>
            <button-collection-item>P1</button-collection-item>
            <button-collection-item>P2</button-collection-item>
        </button-collection>
    `;
}

ButtonCollection.args = DEFAULT_ARGS;
