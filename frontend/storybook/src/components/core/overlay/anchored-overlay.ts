import {argsToAttrs} from "@utils/attributes";
import "@elements/core/overlays/anchored-overlay";
export default {
    title: "Core / Overlays"
}

interface Args {
    autoClose: boolean;
    open: boolean;
    positionY: string,
    positionX: string,
}

const DEFAULT_ARGS:Args = {
    autoClose: true,
    open: true,
    positionY: "top-in",
    positionX: "right-out",
}

export const Overlay = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `
        <div style="margin: 50px">
            <anchored-overlay ${argsToAttrs(props)}>
                <input slot="anchor" style="width: 300px;" value="The overlay is anchored to this input element">
                <div slot="overlay" style="padding: 10px;">Overlay body</div>
            </anchored-overlay>
        </div>
    `;
}

Overlay.args = DEFAULT_ARGS;
Overlay.argTypes = {
    positionY: {
        control: {
            type: 'inline-radio',
            options: ['top-out', 'top-in', 'bottom-out', 'bottom-in'],
        }
    },
    positionX: {
        control: {
            type: 'inline-radio',
            options: ['left-out', 'left-in', 'right-out', 'right-in'],
        }
    },
}
