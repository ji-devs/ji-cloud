import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/widgets/nav/steps-nav";
import "@elements/core/buttons/circle";
import "@elements/core/buttons/rectangle";
import "@elements/module/_common/sidebar";
export default {
    title: "Module / Memory / Edit / Steps"
}

const STR_CONTENT = "Content";
const STR_DESIGN = "Design";
const STR_SETTINGS = "Settings";
const STR_PREVIEW = "Preview";
const STR_DONE = "Done";

interface Args {
    activeStep: number
}

const DEFAULT_ARGS:Args = {
    activeStep: 2
}

export const Sidebar = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {activeStep, content} = props;
    return `
    <module-sidebar slot="sidebar">
            <steps-nav slot="nav" count="4">
                <button-circle slot="slot-1" label="${STR_CONTENT}" ${activeStep == 1 && "active"}>1</button-circle>
                <button-circle slot="slot-2" label="${STR_DESIGN}" ${activeStep == 2 && "active"}>2</button-circle>
                <button-circle slot="slot-3" label="${STR_SETTINGS}" ${activeStep == 3 && "active"}>3</button-circle>
                <button-circle slot="slot-4" label="${STR_PREVIEW}" ${activeStep == 4 && "active"}>4</button-circle>
            </steps-nav>
            ${content ? content : ""}
            <button-rect color="grey" size="small" iconAfter="done" slot="btn">${STR_DONE}</button-rect>
        </module-sidebar>
    `;
}

Sidebar.args = DEFAULT_ARGS;
