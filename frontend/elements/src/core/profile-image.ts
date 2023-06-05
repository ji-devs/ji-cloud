import { LitElement, html, css, customElement, property, state } from "lit-element";
import "@elements/core/images/ui";
import "@elements/core/images/ji";

const COLORS = [
    "#85a6ef",
    "#b38dd0",
    "#f4924e",
    "#ea9498",
]

@customElement("profile-image")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                img-ji, .placeholder {
                    display: inline-block;
                    height: inherit;
                    width: inherit;
                    border-radius: 50%;
                    overflow: hidden;
                    display: inline-grid;
                    place-content: center;
                    color: var(--dark-gray-6);
                }
            `,
        ];
    }

    @property()
    imageId?: string;

    @property()
    givenName: string = "";

    @property()
    familyName: string = "";

    render() {
        return html`
            <style>
                .placeholder {
                    background-color: ${this.getColor()};
                }
            </style>
            ${
                this.imageId === undefined ? html`
                    <span class="placeholder">${this.initials()}</span>
                ` : html`
                    <img-ji
                        lib="user"
                        size="thumb"
                        id="${this.imageId}"
                    ></img-ji>
                `
            }
        `;
    }

    initials(): string {
        const initial_a = this.givenName[0] + "";
        const initial_b = this.familyName[0] + "";
        return initial_a.toUpperCase() + initial_b.toUpperCase();
    }


    getColor(): String {
        const count = countFromString(this.givenName + this.familyName);
        const index = count % COLORS.length;
        return COLORS[index];
    }
}

function countFromString(s: string): number {
    let count = 0;
    for (let i = 0; i < s.length; i++) {
        count += s.charCodeAt(i);
    }
    return count;
}
