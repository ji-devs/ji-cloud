import "@elements/core/page-header/page-header-link";
import { Kind } from "@elements/core/page-header/page-header-link";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Page Header"
}

interface Args {
    kind: Kind;
    active: boolean;
}

const DEFAULT_ARGS:Args = {
    kind: "about",
    active: true,
}

export const PageHeaderLink = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-header-link ${argsToAttrs(props)}></page-header-link>
    `;
}

PageHeaderLink.args = DEFAULT_ARGS;
PageHeaderLink.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ['home', 'content', 'create', 'community', 'classroom', 'about']
        }
    }
}
