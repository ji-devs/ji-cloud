import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {MODE} from "@elements/module/memory/_common/types";

const STR_LABEL = html`Edit your words<br/>on the cards`;

@customElement('step1-sidebar-empty')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100%;
            }
            img-ui {
                margin-bottom: 24px;

            }
            .label {
                font-size: 18px;
                font-weight: 500;
                line-height: 1.22;
                text-align: center;
                color: var(--dark-gray-6);
                margin-left: -16px;
                }

            section {
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                align-items: flex-start;
            }

        `];
    }

    @property()
    mode:MODE = "duplicate";

    render() {
        const {mode} = this;

        return html`
            <section>
                <img-ui path="module/memory/edit/sidebar/jiggling-card-pointer.svg"></img-ui>
                <div class="label">${STR_LABEL}</div>
            </section>
        `
    }
}
