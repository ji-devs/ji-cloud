import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/module-page/grid-resize";
import {Main} from "~/components/module/memory/edit/steps/sections/main/main";
import {Sidebar} from "~/components/module/memory/edit/steps/sections/sidebar/sidebar";
import {Header} from "~/components/module/memory/edit/steps/sections/header";
import {Footer} from "~/components/module/_common/footer";
import {MODE} from "@elements/module/memory/_common/types.ts";
import {Args as CardArgs} from "~/components/module/memory/edit/steps/sections/main/card-pair/card";
import {ThemeKind, ThemeControl} from "~/components/module/_common/theme";
export default {
    title: "Module / Memory / Edit / Steps / Pages"
}

type STEP = 1 | 2 | 3 | 4;
interface Args {
    mode: MODE,
    nCards: number,
    step: STEP,
    theme: ThemeKind
}

const DEFAULT_ARGS:Args = {
    mode: "duplicate",
    nCards: 6,
    step: 1,
    theme: "chalkboard",
}

export const Create = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {mode, nCards, step, theme} = props;


    return `
    <module-page-grid-resize scrollable>
        ${Sidebar({
            activeStep: step,
            mode,
            empty: nCards > 0,
            theme
        })}
        ${Main({
            nCards,
            leftArgs: getLeftArgs(mode, step, theme),
            rightArgs: getRightArgs(mode, step, theme),
        })}
        ${Header()}
        ${Footer()}
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

Create.args = DEFAULT_ARGS;
Create.argTypes = {
    step: {
        control: {
            type: 'inline-radio',
            options: [1, 2, 3, 4]
        }
    },
    mode: {
        control: {
            type: 'inline-radio',
            options: ["duplicate", "words-images", "begins", "lettering"]
        }
    },
    theme: ThemeControl
}
