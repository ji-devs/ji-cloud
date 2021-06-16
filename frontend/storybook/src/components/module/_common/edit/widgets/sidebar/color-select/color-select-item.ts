import "@elements/module/_common/edit/widgets/color-select/color-select-item";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Color Select"
}


interface Args {
    color: string,
    deletable: boolean,
    selected: boolean,
}

const DEFAULT_ARGS:Args = {
    color: "pink",
    deletable: true,
    selected: false,
}

export const colorSelectItem = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <color-select-item ${argsToAttrs(props)}>${
            props.deletable && `<button-icon slot="delete-button" icon="circle-x-blue"></button-icon>`
        }</color-select-item>
    `;
}

colorSelectItem.args = DEFAULT_ARGS;
