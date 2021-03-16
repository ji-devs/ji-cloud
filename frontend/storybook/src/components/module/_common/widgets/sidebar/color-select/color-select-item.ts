import "@elements/module/_common/widgets/color-select/color-select-item";

export default {
    title: "Module / _common / Widgets / Sidebar / Color Select"
}


interface Args {
    color: string,
}

const DEFAULT_ARGS:Args = {
    color: "pink",
}

export const colorSelectItem = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <color-select-item color="${props.color}"></color-select-item>
    `;
}

colorSelectItem.args = DEFAULT_ARGS;
