import { LitElement, html, css, customElement, property } from "lit-element";
import { BaseButton } from "@elements/_styles/buttons";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/rectangle";
import "@elements/module/video/youtube-player";

const STR_NO_SHOW_AGAIN = "Don't show onboarding again";

@customElement("modal-video")
export class _ extends BaseButton {
    static get styles() {
        return [
            css`
                :host {
                    --overlay-z-index: 300;
                    --yt-iframe-position: absolute;
                }

                .overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100vh;
                    opacity: 0.8;
                    background-color: var(--light-blue-3);
                    z-index: var(--overlay-z-index);
                }
                .section {
                    position: fixed;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    z-index: var(--overlay-z-index);

                    width: 75vw;
                    height: auto;
                    border-radius: 16px;
                    -webkit-backdrop-filter: blur(30px);
                    backdrop-filter: blur(30px);
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                    opacity: 1;
                    display: flex;
                    flex-direction: column;
                }

                video-youtube-player {
                    position: relative;
                    width: 100%;
                    height: 0;
                    padding: 0 0 56.25%;
                }

                .close {
                    align-self: flex-end;
                    margin: .5em .5em 0 0;
                }

                .contents {
                    display: flex;
                    flex-direction: column;
                    padding: 32px;
                    flex: 1;
                }

                .noshow {
                    background: transparent;
                    border: none;
                    font-size: 13px;
                    font-weight: 500;
                    color: var(--dark-blue-3);
                    cursor: pointer;
                    align-self: flex-end;
                    padding-top: 32px;
                }
                `,
        ];
    }

    @property({ type: String })
    videoId!: string;

    onAnyClick(evt: MouseEvent) {
        const path = evt.composedPath();
        // Makes sure that only clicking the overlay will trigger a close event.
        if (!path.includes(this.shadowRoot?.getElementById("section") as any)) {
            this.onClose();
        }
    }

    onClose() {
        this.dispatchEvent(new Event("close"));
    }

    render() {
        return html`
            <div class="overlay" @click=${this.onAnyClick}></div>
            <div class="section">
                <button-icon
                    size="x-small"
                    class="close"
                    icon="x"
                    @click=${this.onClose}
                ></button-icon>
                <div class="contents">
                    <video-youtube-player videoid="${this.videoId}"></video-youtube-player>
                </div>
            </div>
        `;
    }
}

