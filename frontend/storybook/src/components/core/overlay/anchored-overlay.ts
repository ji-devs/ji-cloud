import { argsToAttrs } from "@utils/attributes";
import "@elements/core/overlays/anchored-overlay";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Core / Overlays"
}

interface Args {
    open: boolean,
    positionY: string,
    positionX: string,
    autoClose: boolean;
    scrollClose: boolean;
}

const DEFAULT_ARGS: Args = {
    open: true,
    positionY: "bottom-out",
    positionX: "center",
    autoClose: true,
    scrollClose: true,
}

export const AnchoredOverlay = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <div style="padding: 100px 100px 1000px 100px">
            <h1>Something</h1>
            <anchored-overlay ${argsToAttrs(props)}>
                <button slot="anchor" style="padding: 20px">Toggle</button>
                <div slot="overlay">
                    overlay
                </div>
            </anchored-overlay>
        </div>
        <progress></progress>
    `;
}

AnchoredOverlay.args = DEFAULT_ARGS;
AnchoredOverlay.argTypes = {
    positionY: {
        control: {
            type: 'inline-radio',
            options: ['center', 'top-out', 'top-in', 'bottom-out', 'bottom-in'],
        }
    },
    positionX: {
        control: {
            type: 'inline-radio',
            options: ['center', 'left-out', 'left-in', 'right-out', 'right-in'],
        }
    },
}
