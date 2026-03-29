import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty,
} from "lit-element";
import "@elements/core/overlays/anchored-overlay";

const STR_SHALOM = "Shalom";

@customElement("page-header-profile")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    height: 100%;
                    position: relative;
                }
                .wrapper {
                    display: flex;
                    align-items: center;
                    gap: 6px;
                    height: 100%;
                }
                ::slotted([slot="help"]) {
                    display: grid;
                    place-content: center;
                    padding: .5em;
                    cursor: pointer;
                }
                ::slotted([slot="help"]:hover) {
                    border-radius: 50%;
                    background-color: #c4d9f7;
                }
                anchored-overlay,
                anchored-overlay::part(anchor) {
                    height: 100%;
                }
                .main {
                    display: grid;
                    grid-template-columns: auto auto auto;
                    column-gap: 6px;
                    align-items: center;
                    cursor: pointer;
                    height: 100%;
                    position: relative;
                }
                anchored-overlay[open] .main::after {
                    content: "";
                    background-color: var(--light-orange-1);
                    height: 40px;
                    width: 40px;
                    position: absolute;
                    transform: translate(-50%, 50%) rotate(45deg);
                    bottom: 0;
                    z-index: 3;
                    left: 50%;
                    box-shadow: rgb(0 0 0 / 7%) -3px -3px 3px 0px;
                    border-top-left-radius: 8px;
                }
                .main ::slotted([slot="profile-image"]) {
                    display: inline-block;
                    height: 48px;
                    width: 48px;
                    border-radius: 50%;
                    overflow: hidden;
                }
                .main .name {
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-blue-8);
                }
                .main .open-icon {
                    color: var(--dark-gray-3);
                    font-size: 20px;
                    transition: transform 0.2s;
                    transform: rotate(90deg);
                }
                anchored-overlay[open] .main .open-icon {
                    transform: rotate(-90deg);
                }
                @media (max-width: 1024px) {
                    .main {
                        grid-template-columns: auto;
                    }
                    .main .name,
                    .main .open-icon {
                        display: none;
                    }
                    .main ::slotted([slot="profile-image"]) {
                        height: 32px;
                        width: 32px;
                    }
                    anchored-overlay[open] .main::after {
                        bottom: -1.25em;
                    }
                    anchored-overlay::part(overlay) {
                        left: 0 !important;
                        width: 100vw;
                        border-radius: 0;
                        margin-top: 1.25em;
                    }
                }
                .overlay {
                    background-color: var(--light-orange-1);
                    padding: 24px 32px;
                    display: grid;
                    row-gap: 24px;
                }
                @media (min-width: 1025px) {
                    .overlay {
                        min-width: 350px;
                    }
                }
                .overlay .divider {
                    height: 1px;
                    background-color: #8bb5fc;
                }
                .overlay .info {
                    display: grid;
                    grid-template-columns: auto auto;
                    row-gap: 4px;
                    column-gap: 24px;
                }
                .overlay ::slotted([slot="overlay-profile-image"]) {
                    display: inline-block;
                    height: 80px;
                    width: 80px;
                    border-radius: 50%;
                    grid-row: 1 / span 2;
                    overflow: hidden;
                }
                .overlay .info .name {
                    font-size: 20px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    align-self: end;
                    max-width: 200px;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }
                .overlay .info .email {
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    max-width: 200px;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }
                .overlay .user-links {
                    display: grid;
                    align-items: center;
                    grid-template-columns: 18px 1fr;
                    column-gap: 10px;
                    row-gap: 14px;
                }
                .overlay .user-links ::slotted(a) {
                    display: contents;
                    color: var(--main-blue);
                    font-size: 16px;
                    font-weight: 500;
                    text-decoration: none;
                }
                ::slotted([slot="admin"]) {
                    position: absolute;
                    background-color: var(--dark-blue-5);
                    color: #ffffff;
                    text-align: center;
                    width: 124px;
                    line-height: 34px;
                    right: 0;
                    bottom: 0;
                    transform: translateY(100%);
                    font-size: 13px;
                    font-weight: 600;
                    text-decoration: none;
                    border-radius: 0 0 12px 12px;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    email: string = "";

    @internalProperty()
    private open = false;

    private toggleOpen() {
        this.open = !this.open;
    }

    render() {
        return html`
          <div class="wrapper">
            <slot name="help"></slot>
            <anchored-overlay
                .autoClose=${false}
                .open=${this.open}
                @close=${() => (this.open = false)}
                positionX="right-in"
                styled
            >
                <div class="main" slot="anchor" @click=${this.toggleOpen}>
                    <slot name="profile-image"></slot>
                    <span class="name"> ${STR_SHALOM} ${this.name} </span>
                    <span class="open-icon">></span>
                </div>
                <div slot="overlay" class="overlay">
                    <div class="info">
                        <slot name="overlay-profile-image"></slot>
                        <span class="name">${this.name}</span>
                        <span class="email">${this.email}</span>
                    </div>
                    <div class="divider"></div>
                    <div class="user-links">
                        <slot name="user-links"></slot>
                    </div>
                    <div class="divider"></div>
                    <div class="user-links">
                        <slot name="setting-links"></slot>
                    </div>
                    <div class="divider"></div>
                    <div class="logout">
                        <slot name="logout"></slot>
                    </div>
                </div>
            </anchored-overlay>
          </div>
            <slot name="admin"></slot>
        `;
    }
}
