import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/trace/edit-reshape-menu";
import "@elements/module/_common/edit/widgets/trace/edit-reshape-menu-btn";
import "@elements/core/buttons/icon";
export default {
    title: "Module / _COMMON /  edit /Widgets / Trace",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Menu = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
    <trace-edit-reshape-menu style="position: absolute; top: 100px; left: 100px">
        <button-icon icon="circle-x-blue" slot="close"></button-icon>
        <trace-edit-reshape-menu-btn kind="path"></trace-edit-reshape-menu-btn>
        <trace-edit-reshape-menu-btn kind="rect"></trace-edit-reshape-menu-btn>
        <trace-edit-reshape-menu-btn kind="ellipse"></trace-edit-reshape-menu-btn>
        <trace-edit-reshape-menu-btn kind="confirm"></trace-edit-reshape-menu-btn>
    </trace-edit-reshape-menu>
    `;
};

Menu.args = DEFAULT_ARGS;
