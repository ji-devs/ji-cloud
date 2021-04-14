import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { createPopper, Placement, VirtualElement } from '@popperjs/core';
import "@elements/core/buttons/icon";
import "./base";

@customElement("tooltip-confirm")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            :host {
              font-family: Poppins;
            }
            .body {
              font-size: 18px;
              font-weight: 300;
              letter-spacing: -0.18px;
              color: #383838;
            }
            `
        ];
    }

    @property()
    target:Element | VirtualElement | undefined;

    @property()
    placement:Placement = "left";

    @property()
    body:string = "";


    render() {
        const {body, target, placement} = this;

        return html`
            <tooltip-base color="red" .target=${target} .placement=${placement}>
                ${body}
            </tooltip-base>

        `;
    }
}
