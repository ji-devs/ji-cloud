import { LitElement, html, css, customElement, property } from "lit-element";

const STR_HEADER_FIRST = "Create a Professional Development Course";
const STR_HEADER_SECOND = "Build your course by inserting links, files, and videos";

const STR_ADD_LINK = "Add Link";
const STR_UPLOAD_FILE = "Upload file";


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
                /* .column-1 .url {
                    display: grid;
                    grid-row: 1;
                    width: 400px;
                    height: 70px;
                    background-color: var(--light-blue-1);
                }
                .column-2 .name {
                    display: grid;
                    grid-row: 1;
                    height: 65px;
                    width: 400px;
                    grid-template-columns: 1fr min-content;
                    grid-template-rows: min-content 1fr;
                    column-gap: 2px;
                }
                .column-2 .description {
                    display: grid;
                    grid-row: 2;
                    height: 250px;
                    width: 400px;
                    border: 2px solid var(--light-blue-3);
                    border-radius: 15px;
                    grid-template-columns: 1fr min-content;
                    grid-template-rows: min-content 1fr;
                    column-gap: 2px;
                } */
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

                .value-types {
                    display: grid;
                    grid-row: 1;
                    grid-template-columns: 50px 150px 50px 100px;
                    position: start;
                }

                input[type=radio], input[type=checkbox] {
                    margin-right: 2em;
                }

                label {
                    color: var(--dark-blue-1);
                }
            `,
        ];
    }


    @property({ type: Boolean, reflect: false })
    selected: boolean = true;

    private toggleOpen = () => {
        this.selected = !this.selected;
        if (!this.selected) {
            this.dispatchEvent(new Event("closed"));
        }
    };

    render() {
        return html`
           <div class="header">
                <h1>${STR_HEADER_FIRST}</h1>
                <h3>${STR_HEADER_SECOND}</h3>
            </div>
           <div class="main-wrapper">
                <main>
                    <div class="width-holder">
                        <div class="value-types">
                            <input type="radio" name="type" id="rad1" value="link" checked>
                            <label for="rad1">${STR_ADD_LINK}</label>
                            <input type="radio" name="type" id="rad2" value="file">
                            <label for="rad2">${STR_UPLOAD_FILE}</label>
                        </div>
                        <div class="main">
                            <div class="column-1">
                                <slot name="url"></slot>
                                <slot name="file"></slot>
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