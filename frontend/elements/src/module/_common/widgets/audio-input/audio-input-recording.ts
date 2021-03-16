import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("audio-input-recording")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .circle {
                    background-color: #c4dbfe;
                    width: 80px;
                    height: 80px;
                    border-radius: 50%;
                    display: inline-grid;
                    grid-template-columns: repeat(3, 10px);
                    column-gap: 6px;
                    justify-content: center;
                    padding: 28px 0;
                    box-sizing: border-box;
                    align-items: end;
                }
                .line {
                    background-color: var(--main-blue);
                    border-radius: 5px;
                    /* animating height becouse making scale smaller effects the border radius, its fixable but might not be worth the time */
                    height: 24px;
                    animation: spinnerOne 300ms linear infinite alternate;
                }
                .line:nth-child(1) {
                    animation-delay: 0;
                    height: 16px;
                }
                .line:nth-child(2) {
                    animation-delay: 200ms;
                    height: 20px;
                }
                .line:nth-child(3) {
                    animation-delay: 400ms;
                    height: 24px;
                }
                @keyframes spinnerOne {
                    from {
                        height: 24px;
                    }
                    to {
                        height: 16px;
                    }
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="circle">
                <span class="line"></span>
                <span class="line"></span>
                <span class="line"></span>
            </div>
        `;
    }
}
