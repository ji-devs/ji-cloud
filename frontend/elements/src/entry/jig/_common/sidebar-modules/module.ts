import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/text";
import { ModuleKind, GET_STR_MODULE } from "@elements/entry/jig/module-types";
import { collapseStyles } from "./collapse-styles";

@customElement("jig-sidebar-module")
export class _ extends LitElement {
    static get styles() {
        return [
            collapseStyles,
            css`
                section.dragging {
                    transform: rotate(-5deg);
                }

                .dragging .menu, .dragging .decorations, .add-container.dragging {
                    display: none;
                }

                .drag-overlay, section {
                    width: 416px;
                    height: 168px;
                    transition-property: height, width;
                    transition-duration: var(--collapsing-phase-duration);
                    transition-timing-function: linear;
                }
                .drag-overlay {
                    position: absolute;
                    top: 0;
                    left: 0;
                    z-index: 1;
                    cursor: grabbing;
                }
                section {
                    display: flex;
                    cursor: grab;
                    box-sizing: border-box;
                    border: solid 1px transparent;
                    border-left: solid 8px transparent;
                    border-bottom: solid 2px transparent;
                }
                section.selected {
                    border-color: #e7f0fd;
                    background-color: #f8f9fd;
                    border-left-color: var(--main-blue);
                }
                :host([collapsed]) section {
                    height: 136px;
                    width: 72px;
                    transition-delay: var(--fading-phase-duration);
                    border-bottom-color: #e7f0fd;
                }
                .grid-container {
                    margin-top: 23px;
                    display: grid;
                    grid-template-columns: 126px auto auto;
                }

                .left {
                    padding-left: 16px;
                    display: flex;
                    flex-direction: column;
                    text-align: center;
                    width: 0;
                    transition-property: padding-left, width;
                    transition-duration: var(--collapsing-phase-duration);
                    transition-timing-function: linear;
                }
                :host([collapsed]) .left {
                    width: 64px;
                    padding-left: 0;
                    transition-delay: var(--fading-phase-duration);
                }

                .decorations {
                    pointer-events: none;
                }

                .right {
                    margin-left: 16px;
                    display: flex;
                    z-index: 1;
                }

                .title {
                    font-size: 20px;
                    font-weight: bold;
                    line-height: 1.5;
                    color: var(--main-blue);
                }
                .subtitle {
                    font-size: 16px;
                    font-weight: 500;
                    line-height: 1.5;
                    color: #4a4a4a;
                    transition-property: font-size, opacity;
                    transition-duration: var(--collapsing-phase-duration), var(--fading-phase-duration);
                    transition-delay: 0s, var(--collapsing-phase-duration);
                    transition-timing-function: linear;
                }
                :host([collapsed]) .subtitle {
                    font-size: 0px;
                    opacity: 0;
                    transition-delay: var(--collapsing-phase-duration), 0s;
                }
                .icon {
                    margin-top: 8px;
                }
                .window {
                    position: relative;
                    z-index: 1;
                    width: 218px;
                    height: 123px;
                    border-radius: 16px;
                    overflow: hidden;
                }
                .decorations {
                    position: relative;
                    top: 0;
                    left: 0;
                }

                .add-container {
                    position: relative;
                    top: 0px; 
                    left: 0px; 
                    z-index: 1;
                }
                .add {
                    position: absolute;
                    top: -15px; 
                    left: calc(416px - (30px + 17px)); 
                }
                .arm-left, .arm-right, .neck, .head, .torso-columns, .torso-gears, .torso-spring, .feet-spring, .feet-rollers {
                    position: absolute;
                    top: 0;
                    left: 0;
                }

                .arm-left {
                    transform: translate(-35px, -10px);
                }
                .arm-right {
                    transform: translate(190px, -110px);
                }
                .neck {
                    transform: translate(92px, -60px); 
                }
                .head {
                    transform: translate(40px, -200px); 
                }
                .torso-columns {
                    transform: translate(61px, 110px); 
                }
                .torso-gears {
                    transform: translate(60px, 120px); 
                }
                .torso-spring {
                    transform: translate(86px, 110px); 
                }
                .feet-spring {
                    transform: translate(92px, 90px); 
                }
                .feet-rollers{
                    transform: translate(49px, 150px); 
                }
            `,
        ];
    }

    @property({type: Boolean})
    selected: boolean = false;

    // Should be the raw index in the JIG's module list
    // Will be bumped by 1 for display purposes
    @property({type: Number})
    index: number = 0;

    @property()
    module: ModuleKind | "" = "";

    @property({type: Boolean})
    dragging: boolean = false;

    @property({type: Boolean, reflect: true})
    collapsed: boolean = false;

    render() {
        const {selected, index, dragging, module} = this;

        const sectionClasses = classMap({selected, dragging});
        const addContainerClasses = classMap({["add-container"]: true, dragging});

        const title = (index+1).toString().padStart(2, '0');

        const subtitle = module === "" ? "" 
            : GET_STR_MODULE(module);

        const iconPath = module === "" ? "" 
            : `entry/jig/modules/small/${module}.svg`;

        return html`
            <section class="${sectionClasses}">
                <div class="grid-container">
                    <div class="left">
                        <div class="title">${title}</div>
                        ${subtitle === "" ? nothing
                            : html`<div class="subtitle">${subtitle}</div>`
                        }
                        <div class="icon">
                            ${iconPath === "" ? nothing
                                : html`<img-ui path="${iconPath}"></img-ui>`
                            }
                        </div>
                    </div>
                    <div class="middle open-only">
                        <div class="decorations">
                            ${renderDecoration(module, index)}
                        </div>
                        <div class="window">
                            <slot name="window"></slot>
                        </div>
                    </div>
                    <div class="right open-only">
                        <div class="menu">
                            <slot name="menu"></slot>
                        </div>
                    </div>
                    ${dragging ? html`<div class="drag-overlay"></div>` : nothing}
                </div>
            </section>
            <div class="${addContainerClasses} open-only">
                <div class="add">
                    <slot name="add"></slot>
                </div>
            </div>
        `;
    }
}

function renderDecoration(module: ModuleKind | "", index: number) {
    const getImage = (path:string, classes:string) => html`<img-ui class="${classes}" path="entry/jig/jiggling/${path}" />`;

    const renderBottomDecoration = () => {
        return html`
            ${getImage("feet-spring.svg", "feet-spring")}
            ${getImage("yellow/feet-rollers.svg", "feet-rollers")}
        `
    }
    if(module === "cover") {
        return html`
            ${getImage("arm-left.svg", "arm-left")}
            ${getImage("arm-right.svg", "arm-right")}
            ${getImage("neck-spring.svg", "neck")}
            ${getImage("yellow/face.png", "head")}
            ${getImage("torso-columns.svg", "torso-columns")}
        `
    } else {
        switch(index % 3) {
            case 0: return getImage("torso-columns.svg", "torso-columns");
            case 1: return getImage("torso-spring.svg", "torso-spring");
            case 2: return getImage("torso-gears.svg", "torso-gears");
            default: return nothing;
        }
    }
}
