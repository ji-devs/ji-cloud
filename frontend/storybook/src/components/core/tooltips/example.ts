import {argsToAttrs} from "@utils/attributes";
import "@elements/core/overlays/container";
import "@elements/core/overlays/content";
import "@elements/core/overlays/tooltip/confirm";
import "@elements/core/overlays/tooltip/bubble";
import "@elements/core/overlays/tooltip/info";
import "@elements/core/overlays/tooltip/error";
import {Anchor, ContentAnchor, MoveStrategy} from "@elements/core/overlays/content";
export default {
    title: "Core / Tooltips"
}


interface Args {
    contentAnchor: ContentAnchor,
    targetAnchor: Anchor,
    strategy: MoveStrategy,
    marginX: number,
    marginY: number,
    arrowNudge: number,
    kind: "confirm" | "bubble" | "info" | "error"
}

const DEFAULT_ARGS:Args = {
    contentAnchor: "oppositeH",
    targetAnchor: "tr",
    strategy: "track", 
    marginX: 0,
    marginY: 0,
    arrowNudge: 0,
    kind: "bubble"
}

export const Example = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {kind} = props

    const renderConfirm = () => {
        return `<overlay-tooltip-confirm header="testing"  confirmLabel="Confirm" cancelLabel="Cancel" target=".target" arrowNudge="${props.arrowNudge}" targetAnchor="${props.targetAnchor}" contentAnchor=${props.contentAnchor} strategy="${props.strategy}" marginX="${props.marginX}" marginY="${props.marginY}"">
            </overlay-tooltip-confirm>`;
    }
    const renderInfo = () => {
        return `<overlay-tooltip-info showId="debug" title="Title here" body="Body here" closeable target=".target" arrowNudge="${props.arrowNudge}" targetAnchor="${props.targetAnchor}" contentAnchor=${props.contentAnchor} strategy="${props.strategy}" marginX="${props.marginX}" marginY="${props.marginY}" ">
            </overlay-tooltip-info>`;
    }
    const renderBubble = () => {
        return `<overlay-tooltip-bubble target=".target" arrowNudge="${props.arrowNudge}" targetAnchor="${props.targetAnchor}" contentAnchor=${props.contentAnchor} strategy="${props.strategy}" marginX="${props.marginX}" marginY="${props.marginY}" ">
            Body here
            </overlay-tooltip-bubble>`;
    }
    const renderError = () => {
        return `<overlay-tooltip-error target=".target" arrowNudge="${props.arrowNudge}" targetAnchor="${props.targetAnchor}" contentAnchor=${props.contentAnchor} strategy="${props.strategy}" marginX="${props.marginX}" marginY="${props.marginY}" ">
            Body here
            </overlay-tooltip-error>`;
    }
  return `<div class="target" style="position: absolute; top: 30vh; left: 50vw; width: 100rem; height: 100rem; background-color: black; color: white">
            <div style="position: relative; top: 0; left: 0;">
                <div style="position: absolute; top: 0; left: 50rem; width: 1px; height: 100rem; background-color: yellow"></div>
                <div style="position: absolute; top: 50rem; left: 0px; width: 100rem; height: 1px; background-color: yellow"></div>
            </div>
        </div>
        <overlay-container>
            ${kind == "confirm" ? renderConfirm()
             : kind == "bubble" ? renderBubble()
             : kind == "info" ? renderInfo()
             : kind == "error" ? renderError()
             : "" 
            }
        </overlay-container>

    </div>`
}


Example.args = DEFAULT_ARGS;

const targetAnchorOptions = ["tl", "tm", "tr", "ml","mm", "mr", "bl", "bm", "br"];
const contentAnchorOptions = targetAnchorOptions.concat(["oppositeV", "oppositeH", "oppositeVH"]);

const strategies = ["", "dispatchClose", "track"];

Example.argTypes = {
    contentAnchor: {
        control: {
            type: 'inline-radio',
            options: contentAnchorOptions 
        }
    },
    targetAnchor: {
        control: {
            type: 'inline-radio',
            options: targetAnchorOptions 
        }
    },
    strategy: {
        control: {
            type: 'inline-radio',
            options: strategies 
        }
    },
    kind: {
        control: {
            type: 'inline-radio',
            options: ["confirm", "bubble" , "info" , "error"]
        }
    },
}