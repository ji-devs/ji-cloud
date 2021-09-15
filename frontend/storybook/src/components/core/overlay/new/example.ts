import {argsToAttrs} from "@utils/attributes";
import {GridResize} from "../../module-page";
import "@elements/core/overlays/new/container";
import "@elements/core/overlays/new/content";
import {Placement, MoveStrategy} from "@elements/core/overlays/new/content";
import {Container as MenuContainer} from "../../menu/container";
export default {
    title: "Core / Overlays / New"
}

type ContainerOptions = "window" | "#main" | "none";

interface Args {
    flowTargetPlacement: Placement,
    flowTargetStrategy: MoveStrategy,
    flowTargetMargin: number,
    flowTargetContainer: ContainerOptions,
    absoluteTargetPlacement: Placement,
    absoluteTargetStrategy: MoveStrategy,
    absoluteTargetMargin: number,
    absoluteTargetContainer: ContainerOptions, 
}

const DEFAULT_ARGS:Args = {
    flowTargetPlacement: "right-start",
    flowTargetStrategy: "track", 
    flowTargetMargin: 0,
    flowTargetContainer: "window",
    absoluteTargetPlacement: "right-start",
    absoluteTargetStrategy: "track",
    absoluteTargetMargin: 0,
    absoluteTargetContainer: "window",
}

export const Example = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `
        ${GridResize({main: makeMain(props)})}
    `;
}

function makeMain(props:Args) {

    const flowTargetContainer = props.flowTargetContainer === "none" ? "" : props.flowTargetContainer;
    const absoluteTargetContainer = props.absoluteTargetContainer === "none" ? "" : props.absoluteTargetContainer;
  return `<div style="background-color: green; width: 100%; height: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <div style="font-size: 200rem; color: white; text-align: center">Overlay Example</div>
        <div id="flowTarget" style="width: 100rem; height: 100rem; background-color: black; color: white">
            <div style="position: relative; top: 0; left: 0;">
                <div style="position: absolute; top: 0; left: 50rem; width: 1px; height: 100rem; background-color: yellow"></div>
                <div style="position: absolute; top: 50rem; left: 0px; width: 100rem; height: 1px; background-color: yellow"></div>
            </div>
        </div>
        <div id="absoluteTarget" style="position: absolute; top: 200rem; right: 50rem; width: 100rem; height: 100rem; background-color: black; color: white">
            <div style="position: relative; top: 0; left: 0;">
                <div style="position: absolute; top: 0; left: 50rem; width: 1px; height: 100rem; background-color: yellow"></div>
                <div style="position: absolute; top: 50rem; left: 0px; width: 100rem; height: 1px; background-color: yellow"></div>
            </div>
        </div>
        <overlay-container>
            <overlay-content target="#flowTarget" placement="${props.flowTargetPlacement}" strategy="${props.flowTargetStrategy}" margin="${props.flowTargetMargin}" container="${flowTargetContainer}">
                ${MenuContainer()}
            </overlay-content>
            <overlay-content target="#absoluteTarget" placement="${props.absoluteTargetPlacement}" strategy="${props.absoluteTargetStrategy}" margin="${props.absoluteTargetMargin}" container="${absoluteTargetContainer}">
                ${MenuContainer()}
            </overlay-content>
        </overlay-container>

    </div>`
}

function makeContentSquare() {
    return `<div style="width: 100rem; height: 100rem; background-color: black; color: white">
        <div style="position: relative; top: 0; left: 0;">
            <div style="position: absolute; top: 0; left: 50rem; width: 1px; height: 100rem; background-color: red"></div>
            <div style="position: absolute; top: 50rem; left: 0px; width: 100rem; height: 1px; background-color: red"></div>
        </div>
    </div>`
}


Example.args = DEFAULT_ARGS;

const placements = [  "top", "top-start", "top-end", 
    "bottom", "bottom-start" , "bottom-end", 
    "right", "right-start", "right-end", 
    "left", "left-start" , "left-end"
]

const strategies = ["", "dispatchClose", "track"];
const containers = ["window", "#main", "none"]

Example.argTypes = {
    flowTargetPlacement: {
        control: {
            type: 'inline-radio',
            options: placements 
        }
    },
    flowTargetStrategy: {
        control: {
            type: 'inline-radio',
            options: strategies 
        }
    },
    flowTargetContainer: {
        control: {
            type: 'inline-radio',
            options: containers 
        }
    },
    absoluteTargetPlacement: {
        control: {
            type: 'inline-radio',
            options: placements 
        }
    },
    absoluteTargetStrategy: {
        control: {
            type: 'inline-radio',
            options: strategies 
        }
    },
    absoluteTargetContainer: {
        control: {
            type: 'inline-radio',
            options: containers 
        }
    },
}