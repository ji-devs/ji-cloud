import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/instructions-banner";
export default {
    title: "Module / _common"
}

interface Args {
    instructions: string;
}

const DEFAULT_ARGS:Args = {
    instructions: "Instructions Here"
}

export const InstructionsBanner = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {instructions} = props;

    return `<instructions-banner>${instructions}</instructions-banner>`
}

InstructionsBanner.args = DEFAULT_ARGS;
