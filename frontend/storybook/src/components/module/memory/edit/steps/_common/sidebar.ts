import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";

import "@elements/widgets/nav/steps-nav";
import "@elements/widgets/nav/step-nav";
/*
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Widgets / Nav"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const StepsNav = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <steps-nav ${argsToAttrs(props)}>
            <step-nav number="1" label="Themes" completed></step-nav>
            <step-nav number="2" label="Background" completed></step-nav>
            <step-nav number="3" label="Content" active></step-nav>
            <step-nav number="4" label="Preview"></step-nav>
        </steps-nav>
    `;
}

StepsNav.args = DEFAULT_ARGS;
 */
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
    const stepState = (step:number) => {
        if(step == activeStep) {
            return "active";
        } else if (step < activeStep) {
            return "completed";
        } else {
            return "";
        }
    }
    return `
    <module-sidebar slot="sidebar">
        <steps-nav slot="nav" ${argsToAttrs(props)}>
            <step-nav number="1" label="${STR_CONTENT}" ${stepState(1)}></step-nav>
            <step-nav number="2" label="${STR_DESIGN}" ${stepState(2)}></step-nav>
            <step-nav number="3" label="${STR_SETTINGS}" ${stepState(3)}></step-nav>
            <step-nav number="4" label="${STR_PREVIEW}" ${stepState(4)}></step-nav>
        </steps-nav>
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
