import { argsToAttrs } from "@utils/attributes";
import { GridResize } from "../module-page";
import "@elements/core/overlays/container";
import "@elements/core/overlays/content";
import "@elements/core/overlays/drag";

export default {
    title: "Core / Drag",
};

interface Args {
    childHtml: string;
}

const DEFAULT_ARGS: Args = {
    childHtml: "",
};

export const Overlay = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        ${GridResize({ main: makeMain(props) })}
    `;
};

function makeMain(props: Args) {
    const { childHtml, ...containerProps } = props;

    const renderChild = () => {
        if (childHtml == "") {
            return `
            <div style="width: 300px; height: 300px; background-color: beige; display: flex; align-items: center; justify-content: center">
            Content here 
            </div>`;
        } else {
            return childHtml;
        }
    };
    return `<div style="background-color: green; width: 100%; height: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <div style="font-size: 200rem; color: white; text-align: center">Overlay Example</div>
        <overlay-container>
        <overlay-drag x="20000" y="200">
            ${renderChild()}
        </overlay-drag>
        </overlay-container>

    </div>`;
}
