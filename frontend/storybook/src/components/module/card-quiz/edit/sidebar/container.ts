import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/module/card-quiz/edit/sidebar/container";

export default {
    title: "Module / Card Quiz / Edit / Sidebar" 
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `<card-quiz-settings>
        Number of options: <input type="number"></input>
    </card-quiz-settings>`;
}

Container.args= DEFAULT_ARGS;