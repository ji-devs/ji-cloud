import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/module-page/grid-resize";
import {Player} from "~/components/module/memory/play/player";
import {MODE} from "@elements/module/memory/_common/types.ts";
import "@elements/module/_common/preview-header";
import {Args as CardArgs} from "~/components/module/memory/edit/steps/sections/main/card-pair/card";
import {ThemeKind, ThemeControl} from "~/components/module/_common/theme";

const STR_CONTENT = "Content";
const STR_DESIGN = "Design";
const STR_SETTINGS = "Settings";
const STR_PREVIEW = "Preview";
const STR_DONE = "Done";


export default {
    title: "Module / Memory / Edit / Steps / Pages"
}

interface Args {
    mode: MODE,
    nCards: number,
    theme: ThemeKind
}

const DEFAULT_ARGS:Args = {
    mode: "duplicate",
    nCards: 6,
    theme: "chalkboard",
}

export const Preview = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {mode, nCards, theme} = props;


    return `
    <module-page-grid-resize preview>
        <module-preview-header slot="header">
            <steps-nav slot="nav" ${argsToAttrs(props)}>
                <step-nav number="1" label="${STR_CONTENT}" completed></step-nav>
                <step-nav number="2" label="${STR_DESIGN}" completed></step-nav>
                <step-nav number="3" label="${STR_SETTINGS}" completed></step-nav>
                <step-nav number="4" label="${STR_PREVIEW}" active></step-nav>
                </steps-nav>
                <button-rect slot="btn" size="small" iconAfter="arrow">${STR_DONE}</button-rect>
        </module-preview-header>
        ${Player({
            nCards,
        })}
      </module-page-grid-resize>
      `
}

function getLeftArgs(mode: MODE, step: number, theme: ThemeKind):CardArgs {
    return {
        contentMode: "text",
        ioMode: step === 1 ? "edit" : "preview",
        editTarget: false,
        theme: theme,
    }
}
function getRightArgs(mode: MODE, step: number, theme: ThemeKind):CardArgs {
    return {
        contentMode: mode === "duplicate" ? "text" : "image",
        ioMode: step === 1 ? "edit" : "preview",
        editTarget: false,
        theme: theme,
    }
}

Preview.args = DEFAULT_ARGS;
Preview.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ["duplicate", "words-images", "begins", "lettering"]
        }
    },
    theme: ThemeControl
}
