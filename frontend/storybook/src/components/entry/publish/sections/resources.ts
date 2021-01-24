import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/publish/resources";

export default {
  title: 'Entry/Jig / Publish / Sections',
}



interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Resources = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <resources-column></resources-column>
    `
}

Resources.args = DEFAULT_ARGS;