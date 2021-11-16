import { LitElement, html, css, customElement, property } from "lit-element";

const STR_PREVIEW = "Preview";

@customElement("module-header-controller")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    padding: 8px 20px 8px 16px;
                    border-radius: 24px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                }
                section {
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }

                .arrows {
                    /* Not sure why, but this looks more centered */
                    margin-top: 3px;
                    display: flex;
                    gap: 10px;
                }
                img-ui {
                    cursor: pointer;
                }

                .preview {
                    /* Not sure why, but this looks more centered */
                    margin-top: 4px;
                    display: flex;
                    gap: 8px;
                }
                .divider {
                    margin: 0 16px;
                    height: 32px;
                    border: solid 1px #606060;
                }
            `,
        ];
    }

    onUndo() {
        const { undoable } = this;
        if (undoable) {
            this.dispatchEvent(
                new CustomEvent("custom-string", {
                    detail: { value: "undo" },
                })
            );
        }
    }
    onRedo() {
        const { redoable } = this;
        if (redoable) {
            this.dispatchEvent(
                new CustomEvent("custom-string", {
                    detail: { value: "redo" },
                })
            );
        }
    }
    onPreview() {
        this.dispatchEvent(
            new CustomEvent("custom-string", {
                detail: { value: "preview" },
            })
        );
    }
    @property({ type: Boolean, reflect: true })
    undoable: boolean = false;

    @property({ type: Boolean, reflect: true })
    redoable: boolean = false;

    render() {
        const { undoable, redoable } = this;
        const undoButton = undoable ? "undo" : "undo-disabled";
        const redoButton = redoable ? "redo" : "redo-disabled";
        return html`
            <section>
                <div class="arrows">
                    <img-ui
                        @click=${this.onUndo}
                        path="module/_common/edit/header/${undoButton}.svg"
                    ></img-ui>
                    <img-ui
                        @click=${this.onRedo}
                        path="module/_common/edit/header/${redoButton}.svg"
                    ></img-ui>
                </div>
                <div class="divider"></div>
                <div class="preview" @click=${this.onPreview}>
                    <img-ui path="module/_common/edit/header/play.svg"></img-ui>
                    <div class="preview-label">${STR_PREVIEW}</div>
                </div>
            </section>
        `;
    }
}
/*
 */
