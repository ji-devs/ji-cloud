import { argsToAttrs, deleteNone } from "@utils/attributes";
import "@elements/core/modals/confirm";

export default {
    title: "Core / Modals",
};

const DEFAULT_ARGS: Args = {
    title: "Warning",
    body: "Are you sure you want to delete this thing?",
    cancel_text: "Cancel",
    confirm_text: "Confirm",
    dangerous: true,
};

export const Confirm = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;


    const cancel = () => {
        console.log('foo')
    }

    const confirm = () => {
        console.log('confirm')
    }

    return `
        <div @cancel="${cancel}" @confirm="${confirm}">
            <modal-confirm ${argsToAttrs(props)}></modal-confirm>
        </div>
    `;
};

//Continuing the previous example
Confirm.args = DEFAULT_ARGS;
