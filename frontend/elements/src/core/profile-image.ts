import { LitElement, html, css, customElement, property, state } from "lit-element";
import "@elements/core/images/ui";
import "@elements/core/images/ji";
import { nothing } from "lit-html";

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
                :host {
                    display: inline-grid;
                }
                img-ji, .placeholder, .badge {
                    grid-column: 1;
                    grid-row: 1;
                }
                img-ji, .placeholder {
                    display: inline-block;
                    height: inherit;
                    width: inherit;
                    border-radius: 50%;
                    overflow: hidden;
                    display: inline-grid;
                    place-content: center;
                    color: var(--dark-gray-6);
                    font-size: 32px;
                    font-weight: 600;
                }
                .badge {
                    align-self: end;
                    justify-self: end;
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

    @property()
    badge?: "master-teacher" | "ji-team";

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
            ${
                this.badge === "master-teacher" ? html`
                    <img-ui class="badge" path="core/profile-image/badge-master-teacher.svg">
                ` : this.badge === "ji-team" ?  html`
                    <img-ui class="badge" path="core/profile-image/badge-ji-team.svg">
                `: nothing
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
