import "@elements/core/page-header/page-header-profile";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Page Header"
}

interface Args {
    name: string;
}

const DEFAULT_ARGS:Args = {
    name: "Some Name",
}

export const PageHeaderProfile = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-header-profile ${argsToAttrs(props)}></page-header-profile>
    `;
}

PageHeaderProfile.args = DEFAULT_ARGS;
