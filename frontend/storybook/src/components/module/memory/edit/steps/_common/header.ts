import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/_common/header";
export default {
    title: "Module / Memory / Edit / Steps"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `<steps-header slot="header">
    <div slot="controller">Undo/Redo | Preview</div>
    </steps-header>`
}

Header.Args = DEFAULT_ARGS;
