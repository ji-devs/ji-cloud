import { LitElement, html, css, customElement, property } from "lit-element";
import { ThemeId } from "@elements/_themes/themes";
import { getContentStyleColors } from "@elements/module/_groups/cards/helpers";
import { applyButtonRoleEvents } from "@elements/core/aria-helpers";

@customElement("add-pair")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --card-size: 160px;
                    --border-size: 1px;
                    --img-padding: 10px;

                    --theme-border-color: --theme-blank-cards-border-color;
                    --theme-border-color-light: --theme-blank-cards-border-color-light-hsl;

                    justify-self: left;
                    padding: 27px;
                    margin-top: 24px;
                }

                section {
                    width: var(--card-size);
                    height: var(--card-size);

                    cursor: pointer;
                    transition: background 0.35s;

                    border-style: solid;
                    border-radius: 16px;
                    border-width: var(--border-size);

                    border-color: hsl(var(--theme-border-color-light));
                    background-color: var(--light-blue-2);
                    margin: 2px;

                    display: grid;
                    align-content: center;
                    justify-content: center;
                }

                section:hover {
                    background-color: var(--light-blue-3);
                }

                section:focus {
                    outline: 3px hsl(var(--theme-border-color-light)) solid;
                }

                img-ui {
                    margin: auto;
                }

                .front {
                    display: grid;
                    height: 100%;
                    width: 100%;
                }
            `,
        ];
    }

    @property()
    theme: ThemeId = "blank";

    connectedCallback() {
        super.connectedCallback();

        applyButtonRoleEvents(this);
    }

    updated() {
        const { theme } = this;
        const styleConfig = getContentStyleColors(theme);
        this.style.setProperty("--theme-border-color", styleConfig.borderColor);
        this.style.setProperty("--theme-border-color-light", styleConfig.borderColorLight);
    }

    render() {
        return html`
            <section role="button" focusable="true" tabindex="0">
                <div class="front">
                    <img-ui path="core/buttons/icon/circle-+-blue.svg"></img-ui>
                    <div class="label">Add pair</div>
                </div>
            </section>
        `;
    }
}
