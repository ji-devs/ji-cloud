import "@elements/entry/jig/gallery/create";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Entry / Jig / Gallery"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Create = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-gallery-create ${argsToAttrs(props)}></jig-gallery-create>
    `;
}

Create.args = DEFAULT_ARGS;
