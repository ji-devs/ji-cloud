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
                  display: flex;
                  padding: 25px;
                  overflow: auto;
                  box-sizing: border-box;
                  flex-direction: column;
                  row-gap: 65px;
                  width: 100%
                  height: 100%
                }
              
                .main-wrapper {
                  display: flex;
                  justify-content: center;
                  align-items: center;
                }
              
                main {
                  display: flex;
                  justify-content: center;
                  place-content: center;
                  background-color: var(--white);
                  padding: 38px;
                  border-radius: 32px;
                  box-shadow: 0 3px 8px 0 rgba(0, 0, 0, 0.08);
                  width: 100%;
                  height: 100%;
                }

                .header {
                  display: flex;
                  flex-direction: column;
                  position: sticky;
                  top: 0;
                  z-index: 1;
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
                  display: flex;
                  flex-direction: column;
                  justify-content: flex-start;
                  height: 100%;
                  width: 100%;
                  position: relative;
                }
              
                .main {
                  display: flex;
                  height: 100%;
                  column-gap: 40px;
                  flex-direction: row;
                  align-items: stretch;
                  height: 60vh;
                  max-height: 50vh;
                }

                .column-1,
                .column-2 {
                  display: flex;
                  flex-direction: column;
                  flex-grow: 1;
                  row-gap: 40px;
                  flex: 1 0 0;
                  height: 100%;
                }

                .column-1 > *,
                .column-2 > * {
                    margin-bottom: 20px;
                }

                .column-2 > * {
                  margin-bottom: 40px;
                }

                ::slotted([slot="name"]) {
                  /* height: 100%; */
                  /* max-height: 13%; */
                }

                ::slotted([slot="description"]) {
                  height: 100%;
                }

                ::slotted([slot="description"]) {
                  height: 100%;
                }

                ::slotted([slot="unit-play"]) {
                  display: grid;
                  height: 100%;
                  border-radius: 16px;
                  overflow: hidden;
                  object-fit: contain;
                }

                main .save ::slotted([slot="add"]) {
                  /* max-height: 45px; */
                }
              
                main .save {
                  display: flex;
                  justify-content: center;
                  align-items: center;
                  width: 100%;
                  z-index: 1;
                  padding-top: 40px;
                }

                .controls {
                  cursor: pointer;
                  display: flex;
                  column-gap: 20px;
                  align-items: center;
                  padding-bottom: 15px;
                  font-size: large;
                  color: var(--light-blue-6);
                  font-weight: 400;
                }

                label {
                  color: var(--dark-blue-1);
                }


                .player-window img-ji {
                    display: grid;
                    border-radius: 16px;
                    justify-content: center;
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
                        <slot name="youtube-select"></slot>
                        <slot name="link-select"></slot>
                        <slot name="file-select"></slot>
                    </div>
                    <div class="main">
                        <div class="column-1">
                            <slot name="body-input"></slot>
                            <slot name="unit-play"></slot>
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

