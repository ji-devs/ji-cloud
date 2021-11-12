import { argsToAttrs } from "@utils/attributes";
import { mapToString, arrayCount } from "@utils/array";
import "@elements/module/_common/edit/widgets/settings/container";
import "@elements/module/_common/edit/widgets/settings/line";
import { Button } from "./button";

export default {
    title: "Module / _COMMON /  edit / Widgets / Sidebar / Settings ",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Container = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
	<module-settings-container>
            <module-settings-line ${argsToAttrs(props)}>
	    	${Button({ offsetContainer: false })} ${Button({ offsetContainer: false })}
            </module-settings-line>
            <module-settings-line ${argsToAttrs(props)}>
	    	${Button({ offsetContainer: false, kind: "score" })} ${Button({
        offsetContainer: false,
        kind: "score-off",
    })}
            </module-settings-line>
            <module-settings-line ${argsToAttrs(props)}>
	    	${Button({ offsetContainer: false, kind: "card-single" })} ${Button({
        offsetContainer: false,
        kind: "card-single",
    })}
            </module-settings-line>
	</module-settings-container>
    `;
};

Container.args = DEFAULT_ARGS;
