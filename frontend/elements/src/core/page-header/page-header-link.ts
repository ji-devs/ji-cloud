import { LitElement, html, css, customElement, property } from "lit-element";

export type Kind =
    | "home"
    | "content"
    | "create"
    | "community"
    | "classroom"
    | "about";

const STR_LABEL_LOOKUP: {
    [key in Kind]: string;
} = {
    ["home"]: "Home",
    ["content"]: "Content",
    ["create"]: "Create",
    ["community"]: "Community",
    ["classroom"]: "Classroom",
    ["about"]: "About JI",
};

@customElement("page-header-link")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: contents;
                }
                a {
                    text-decoration: none;
                    color: inherit;
                    display: grid;
                }
                a:hover {
                    background-color: var(--light-blue-1);
                }
                .center {
                    border-top: solid 6px transparent;
                    display: flex;
                    flex-direction: column;
                    row-gap: 4px;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                    font-weight: 500;
                    font-size: 14px;
                    padding-right: 16px;
                    padding-left: 16px;
                }
                img-ui {
                    align-items: center;
                }
                @media (min-width: 1920px) {
                    .center {
                        font-size: 16px;
                        margin: 0 auto;
                        align-items: center;
                    }
                }
                :host([active]) .center {
                    border-color: #fd7076;
                    color: var(--dark-red-1);
                }
            `,
        ];
    }

    @property()
    kind: Kind = "home";

    @property()
    href: string = "";

    @property()
    target: string = "";

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    render() {
        const path = `core/page-header/nav-icon-${this.kind}${
            this.active ? "-active" : ""
        }.svg`;
        return html`
            <a href=${this.href} .target=${this.target}>
                <div class="center">
                    <img-ui path="${path}"></img-ui>
                    <span>${STR_LABEL_LOOKUP[this.kind]}</span>
                </div>
            </a>
        `;
    }
}
