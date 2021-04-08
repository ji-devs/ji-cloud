import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/sidebar-body";
import {SingleList} from "./widgets/single-list";
import {ImageSelect} from "~/components/module/_common/widgets/sidebar/image-search/image-select";

const STR_CLEAR = "Clear list";

export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step1"
}

interface Args {
    tab: "text" | "image"
}

const DEFAULT_ARGS:Args = {
    tab: "image"
}

export const WordsAndImages = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {tab} = props;

    return `

        <menu-tabs slot="content">
            <menu-tab slot="tabs" ${tab === "text" ? "active" : ""}>
                <menu-tab-title kind="text"></menu-tab-title>
            </menu-tab>
            <menu-tab slot="tabs" ${tab === "image" ? "active" : ""}>
                <menu-tab-title kind="image" active></menu-tab-title>
            </menu-tab>
            <module-sidebar-body slot="body">
                ${tab === "text" ? SingleList() : ImageSelect()}
            </module-sidebar-body>
        </menu-tabs>
            `
}

WordsAndImages.args = DEFAULT_ARGS;
WordsAndImages.argTypes = {
    tab: {
        control: {
            type: 'inline-radio',
            options: ['text', 'image']
        }
    }
}
