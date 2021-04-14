import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { createPopper, Placement, VirtualElement } from '@popperjs/core';
import "@elements/core/buttons/icon";
import "./base";

@customElement("tooltip-error")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            :host {
              font-family: Poppins;
            }
            .body {
                font-size: 16px;
                color: var(--dark-gray-6);
            }
            `
        ];
    }

    @property()
    target:Element | VirtualElement | undefined;

    @property()
    placement:Placement = "right";

    @property()
    body:string = "";


    render() {
        const {body, target, placement} = this;

        return html`
            <tooltip-base color="red" .target=${target} .placement=${placement}>
                <div class="body">${body}</div>
            </tooltip-base>

        `;
    }
}
