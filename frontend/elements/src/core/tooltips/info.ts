import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { createPopper, Placement, VirtualElement } from '@popperjs/core';
import "@elements/core/buttons/icon";
import "./base";

const STR_NO_SHOW_AGAIN = "Don't show again";

@customElement("tooltip-info")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            :host {
              font-family: Poppins;
            }
            .title {
              font-size: 28px;
              font-weight: 900;
              color: var(--dark-blue-4);
            }
            .body {
              font-size: 18px;
              font-weight: 300;
              letter-spacing: -0.18px;
              color: #383838;
            }
            .noshow {
              font-size: 16px;
              font-weight: 500;
              color: var(--main-blue);
              cursor: pointer;
            }
            `
        ];
    }

    onClose() {
        console.log("TODO - close...");
    }
    @property()
    target:Element | VirtualElement | undefined;

    @property()
    placement:Placement = "left";

    @property()
    title:string = "";

    @property()
    body:string = "";

    @property()
    showId:string | "debug" = "";

    @property({type: Boolean})
    closeable:boolean = false;

    render() {
        const {title, body, showId, target, placement, closeable} = this;

        return html`
            <tooltip-base color="beige" .target=${target} .placement=${placement}>
                ${closeable ? renderClose(this.onClose) : nothing}
                ${title !== "" ? html`<div class="title">${title}</div>` : nothing}
                ${body !== "" ? html`<section class="body">${body}</section>` : nothing}
                ${showId !== "" ? renderShowId(showId, this.onClose) : nothing}       
            </tooltip-base>

        `;
    }
}

function renderClose(onClose: () => any) {
    return html`<button-icon class="close" icon="circle-x-blue" @click=${onClose}></button-icon>`
}

function renderShowId(showId:string, onClose: () => any) {
    const onClick = () => {
        if(showId === "debug") {
            console.log("skipping showId action because it's debug");
        }

        onClose();
        console.log(`check ${showId}`);
    }
    return html`<div @click=${onClick} class="noshow">${STR_NO_SHOW_AGAIN}<div>`;
}
