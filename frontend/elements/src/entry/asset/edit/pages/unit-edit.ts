import { LitElement, html, css, customElement, property } from "lit-element";

const STR_HEADER_FIRST = "Create a Professional Development Course";
const STR_HEADER_SECOND = "Build your course by inserting links, files, and videos";

const STR_ADD_LINK = "Add Link";
const STR_UPLOAD_FILE = "Upload file";
const STR_YOUTUBE = "Youtube link";


@customElement("unit-edit")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    padding: 50px;
                    height: 100dvh;
                    overflow: auto;
                    box-sizing: border-box;
                    overflow: auto;
                }
                .main-wrapper {
                    display: grid;
                    place-content: center;
                    max-width: 100%;
                    max-height: 100%;
                }
                main {
                    display: grid;
                    place-content: center;
                    background-color: var(--white);
                    padding: 38px;
                    border-radius: 32px;
                    box-shadow: 0 3px 8px 0 rgba(0, 0, 0, 0.08);
                    position: fixed;
                    left: 30%;
                }

                h1 {
                    font-size: 32px;
                    font-weight: 900;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                h3 {
                    font-weight: 500;
                    color: #4a4a4a;
                    margin: 0;
                }

                .width-holder {
                    display: grid;
                    grid-template-rows: auto 4fr auto;
                    row-gap: 15px;
                    width: 900px;
                    max-width: 1300px;
                }
                .main {
                    display: grid;
                    grid-row: 2;
                    column-gap: 35px;
                }
                .column-1 {
                    display: grid;
                    grid-row: 2;
                    grid-template-rows: repeat(2, auto);
                    row-gap: 20px;
                } 
                .column-2 {
                    display: grid;
                    grid-row: 2;
                    grid-template-rows: 60px 300px;
                    row-gap: 40px;
                } 
                .save {
                    display: grid;
                    grid-row: 3;
                    place-content: center;
                }

                .controls {
                    cursor: pointer;
                    display: flex;
                    column-gap: 6px;
                    align-items: center;
                }

                label {
                    color: var(--dark-blue-1);
                }
            `,
        ];
    }

    render() {
        return html`
        <div class="header">
            <h1>${STR_HEADER_FIRST}</h1>
            <h3>${STR_HEADER_SECOND}</h3>
        </div>
        <div class="main-wrapper">
            <main>
                <div class="width-holder">
                    <div class="controls">
                        <slot name="link-select"></slot>
                        <slot name="file-select"></slot>
                        <slot name="youtube-select"></slot>
                    </div>
                    <div class="main">
                        <div class="column-1">
                            <slot name="body-input"></slot>
                            <slot name="body-preview"></slot>
                        </div>
                        <div class="column-2">
                            <slot name="name"></slot>
                            <slot name="description"></slot>
                        </div>
                    </div>
                    <div class="save">
                        <slot name="add"></slot>
                    </div>
                </div>
            </main>
        </div>
        `;
    }
}

// <img-ui path="entry/jig/publish/link-icon-pink.svg"></img-ui> 

// <!-- <input type='radio' name='a' checked/>
// <input type='radio' name='a'/> -->

