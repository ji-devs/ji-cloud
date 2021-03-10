import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/widgets/nav/steps-nav";
import "@elements/core/buttons/circle";
export default {
    title: "Widgets / Nav"
}

interface Args {
    count: number,
}

const DEFAULT_ARGS:Args = {
    count: 4,
}

export const StepsNav = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {...navProps} = props;

    return `
        <steps-nav ${argsToAttrs(navProps)}>
            ${mapToString(arrayCount(navProps.count), i => {
                return `<button-circle slot="slot-${i}" label="button ${i}">${i}</button-circle>`;
            })}
        </steps-nav>
    `;
}

StepsNav.args = DEFAULT_ARGS;
