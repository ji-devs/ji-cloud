import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/choose/pages/landing";
import "@elements/widgets/module-page/grid-resize";
import "@elements/module/memory/edit/steps/_common/sidebar";
import {MODE} from "@elements/module/memory/edit/choose/card";
import {Duplicate as DuplicateSidebar} from "../sidebar/duplicate";
import {Duplicate as DuplicateMain} from "../main/duplicate";
import {Header} from "../../_common/header";
import {Footer} from "../../_common/footer";
import {Sidebar} from "../../_common/sidebar";
export default {
    title: "Module / Memory / Edit / Steps / Step1 / Pages"
}

interface Args {
    mode:MODE
}

const DEFAULT_ARGS:Args = {
    mode: "duplicate"
}

export const Step1 = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {mode} = props;


    return `
    <module-page-grid-resize scrollable>
        ${Sidebar({
            activeStep: 1,
            content: mode === "duplicate" ? DuplicateSidebar() : undefined
        })}
        ${mode === "duplicate" ? DuplicateMain() : ""}
        ${Header()}
        ${Footer()}
      </module-page-grid-resize>
      `
}

Step1.Args = DEFAULT_ARGS;
Step1.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ["duplicate", "words-images", "begins", "lettering"]
        }
    }
}
