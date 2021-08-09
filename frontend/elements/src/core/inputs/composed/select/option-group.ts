import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/inputs/primitives/select/base-option-group";


@customElement("input-select-option-group")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .anchor {
                    display: flex;
                    justify-content: space-between;
                    column-gap: 10px;
                    padding: 4px 10px;
                }
                .anchor:hover,
                base-option-group[active] .anchor,
                base-option-group[active-within] .anchor,
                base-option-group[selected-within] .anchor {
                    background-color: var(--light-blue-2);
                }
                base-option-group::part(overlay) {
                    border-radius: 14px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: white;
                    padding: 8px 0;
                }
                .arrow {
                    color: var(--main-blue)
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <base-option-group>

                <div class="anchor" slot="anchor">
                    <slot name="label">
                        <span>${this.label}</span>
                    </slot>
                    <span class="arrow">></span>
                </div>

                <slot></slot>

            </base-option-group>
        `;
    }
}
