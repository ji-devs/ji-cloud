import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/widgets/nav/steps-nav";
import "@elements/core/buttons/circle";
import "@elements/core/buttons/rectangle";
import "@elements/module/_common/sidebar";
import {MODE} from "@elements/module/memory/_common/types.ts";
import {Empty as Step1Empty} from "../step1/sidebar/empty";
import {Duplicate as Step1Duplicate} from "../step1/sidebar/duplicate";
export default {
    title: "Module / Memory / Edit / Steps"
}

const STR_CONTENT = "Content";
const STR_DESIGN = "Design";
const STR_SETTINGS = "Settings";
const STR_PREVIEW = "Preview";
const STR_DONE = "Done";

interface Args {
    activeStep: number,
    mode: MODE,
    empty: boolean,
}

const DEFAULT_ARGS:Args = {
    activeStep: 2,
    mode: "duplicate",
    empty: false 
}

export const Sidebar = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {activeStep, mode, empty} = props;
    return `
    <module-sidebar slot="sidebar">
            <steps-nav slot="nav" count="4">
                <button-circle slot="slot-1" label="${STR_CONTENT}" ${activeStep == 1 && "active"}>1</button-circle>
                <button-circle slot="slot-2" label="${STR_DESIGN}" ${activeStep == 2 && "active"}>2</button-circle>
                <button-circle slot="slot-3" label="${STR_SETTINGS}" ${activeStep == 3 && "active"}>3</button-circle>
                <button-circle slot="slot-4" label="${STR_PREVIEW}" ${activeStep == 4 && "active"}>4</button-circle>
                </steps-nav>
                ${getContents(activeStep, mode, empty)}
                ${getButton(activeStep, mode, empty)}
                
        </module-sidebar>
    `;
}

Sidebar.args = DEFAULT_ARGS;

function getContents(step: number, mode: MODE, empty: boolean) {
    switch(mode) {
        case "duplicate": {
            switch(step) {
                case 1: {
                    if(empty) {
                        return Step1Empty({mode});
                    } else {
                        return Step1Duplicate();
                    }
                }
                default: return ""
            }
        }
        default: return ""
    }
}

function getButton(step: number, mode: MODE, empty: boolean) {
    return empty 
        ? "" 
        : `<button-rect color="grey" size="small" iconAfter="done" slot="btn">${STR_DONE}</button-rect>`
}
