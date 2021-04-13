import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";

import "@elements/widgets/nav/steps-nav";
import "@elements/widgets/nav/step-nav";
import "@elements/core/buttons/rectangle";
import "@elements/module/_common/sidebar";
import {MODE} from "@elements/module/memory/_common/types";
import {Empty as Step1Empty} from "./step1/empty";
import {Duplicate as Step1Duplicate} from "./step1/duplicate";
import {WordsAndImages as Step1WordsAndImages} from "./step1/words-and-images";
import {Translate as Step1Translate} from "./step1/translate";
import {Container as Step2} from "./step2/container";
import {ThemeKind} from "~/components/module/_common/theme";
export default {
    title: "Module / Memory / Edit / Steps / Sections"
}

const STR_CONTENT = "Content";
const STR_DESIGN = "Design";
const STR_SETTINGS = "Settings";
const STR_PREVIEW = "Preview";

interface Args {
    activeStep: number,
    mode: MODE,
    empty: boolean,
    theme: ThemeKind, 
}

const DEFAULT_ARGS:Args = {
    activeStep: 2,
    mode: "duplicate",
    empty: false,
    theme: "chalkboard",
}

export const Sidebar = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {activeStep, mode, empty, theme} = props;
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
                ${getContents(activeStep, mode, empty, theme)}
                
        </module-sidebar>
    `;
}

Sidebar.args = DEFAULT_ARGS;

function getContents(step: number, mode: MODE, empty: boolean, theme: ThemeKind) {
    switch(step) {
        case 1: {
            if(empty) {
                return Step1Empty({mode});
            } else {
                switch(mode) {
                    case "duplicate": return Step1Duplicate();
                    case "words-images": return Step1WordsAndImages();
                    case "translate": return Step1Translate();
                    default: return "";
                }
            }
        }
        case 2: return Step2({theme});
        default: return "";
    }
}
