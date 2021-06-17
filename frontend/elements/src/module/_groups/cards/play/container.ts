import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";
import { mediaUi } from "@utils/path";
import {ThemeKind} from "@elements/_themes/themes";
import {playerBackPath} from "@elements/module/_groups/cards/helpers";


@customElement("play-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                section {
                    display: grid;
                    grid-template-columns: 1fr calc(1432rem * (1920/1719));
                    column-gap: calc(56rem * (1920/1719));
                    width: 100%;
                    height: 100%;
                    background-size: cover;
                }
            `,
        ];
    }

      @property()
      theme:ThemeKind = "blank";
    render() {
        const {theme} = this;

        const style = `background-image: url("${mediaUi(playerBackPath(theme))}");`;

        console.log(style);

        return html`
            <section style="${style}">
                <slot name="sidebar"></slot>
                <div>
                    <slot name="header"></slot>
                    <slot name="main"></slot>
                </div>
            </section>
        `
    }
}
