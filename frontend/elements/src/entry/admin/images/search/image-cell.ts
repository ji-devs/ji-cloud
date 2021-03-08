import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from 'lit-html';
import {classMap} from "lit-html/directives/class-map";
export type Mode = "saved" | "published" ;

@customElement('search-image-cell')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: block;
            }
            :host, .image, slot[name=image]::slotted(*) {
                width: 256px;
            }

            .image {
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                margin-bottom: 10px;
            }

            .active .image {
                  border-radius: 8px;
                  border: solid 4px #5590fc;
            }

            slot[name=image]::slotted(*) { 
                height: 190px;
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
            .circle.red {
                background-color: #e36486;
            }

            .name-line {
                display: flex;
                align-items: center;
            }

            .name {
                text-align: center;
                width: 100%;
                font-size: 16px;
                font-weight: 500;
                font-stretch: normal;
                font-style: normal;
                line-height: 1.25;
                letter-spacing: -0.16px;
            }

            .active .name {
                color: #5590fc;
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
            <section class="${classMap({active})}"> 
                <div class="image">
                    <slot name="image"></slot>
                </div>
                <div class="name-line">
                    <div class="${circleClasses}"></div>
                    <div class="name">${name}</div>
                </div>
            </section>
  `;
    }
}
