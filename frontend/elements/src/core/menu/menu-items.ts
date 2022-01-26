import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("menu-items")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --items-left: 0px;
                    --items-top: 0px;
                }
                :host {
                    position: absolute;
                    left: var(--items-left);
                    top: var(--items-top);
                    border-radius: 8px;
                    -webkit-backdrop-filter: blur(30px);
                    backdrop-filter: blur(30px);
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.2);
                    background-color: var(--white);
                    display: inline-block;
                    padding: 14px 16px;
                }
            `,
        ];
    }

    @property()
    target?: HTMLElement;

    @property({ type: Number })
    offsetVertical: number = 8; // Copied from kebab.ts

    @property({ type: Number })
    offsetHorizontal: number = 50; // Copied from kebab.ts

    updatePosition() {
        const rect = this.target?.getBoundingClientRect();
        if (rect) {
            const { top, left } = rect;
            this.style.setProperty('--items-left', `${left + this.offsetHorizontal}px`);
            this.style.setProperty('--items-top', `${top + this.offsetVertical}px`);
        }
    }

    firstUpdated(_changed: any) {
        this.updatePosition();
    }

    render() {
        console.log("Rending menu-items");
        return html`
            <slot></slot>
        `;
    }
}
