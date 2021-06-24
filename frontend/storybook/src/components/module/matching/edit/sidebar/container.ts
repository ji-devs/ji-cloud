import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/module/matching/edit/sidebar/container";

export default {
    title: "Module / Matching / Edit / Sidebar" 
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `<matching-settings>
    </matching-settings>`;
}

Container.args= DEFAULT_ARGS;