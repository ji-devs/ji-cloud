import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from 'lit-html';
import {classMap} from "lit-html/directives/class-map";
export type Mode = "saved" | "published" ;
@customElement('search-image-display')


export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: block;
                width: 256px;
            }

            .image {
                display: block;
                width: 256px;
                height: 256px;
                background-color: red;
            }
            slot[name=image]::slotted(*) { 
                width: 256px;
                height: 256px;
                object-fit: contain;
            }
            .circle {
              width: 16px;
              height: 16px;
              border-radius: 50%;
              margin-right: 10px;
            }
            .circle.green {
              background-color: #6eca90;
            }

            .name-line {
                display: flex;
                align-items: center;
            }

            .name {
                text-align: center;
                width: 100%;
            }

    `];
    }

    @property()
    name: string = "";

    @property({ type: Boolean })
    active: boolean = false;

    @property()
    mode: Mode = "saved";

    render() {
        const { active,name , mode } = this;

        const circleClasses = classMap({
            circle: true,
            green: mode === "published",
            red: mode !== "published",
        });

        return html`
            <div class="image">
                <slot name="image"></slot>
            </div>
            <div class="name-line">
                <div class="${circleClasses}"></div>
                <div class="name">${name}</div>
            </div>
  `;
    }
}
