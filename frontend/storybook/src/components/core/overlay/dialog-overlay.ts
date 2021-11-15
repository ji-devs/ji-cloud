import { argsToAttrs } from "@utils/attributes";
import "@elements/core/overlays/dialog-overlay";
export default {
    title: "Core / Overlays",
};

interface Args {
    autoClose: boolean;
    open: boolean;
}

const DEFAULT_ARGS: Args = {
    autoClose: true,
    open: true,
};

export const DialogOverlay = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <dialog-overlay ${argsToAttrs(props)} style="background:#00000030;">
            <div style="background:#b86e0b;">Overlay content</div>
        </dialog-overlay>
    `;
};

DialogOverlay.args = DEFAULT_ARGS;
