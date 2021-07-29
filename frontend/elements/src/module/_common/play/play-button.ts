import { LitElement, html, css, customElement, property, internalProperty } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("module-play-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
	    :host {
		    position: absolute;
		    top: 0;
		    left: 0;
		    display: flex;
		    align-items: center;
		    justify-content: center;
		    background-color: rgba(0,0,0,.5);
		    width: 100%;
		    height: 100%;
		    cursor: pointer;
	    }
                button {
                    width: 160rem;
                    height: 160rem;
                    box-shadow: 0 3px 80px 0 rgba(0, 0, 0, 0.4);
                    border-radius: 50%;
                    background-color: #f84f57;
                    border: none;
                    cursor: pointer;
                }
                img-ui {
                    height: 80rem;
                }

		.hidden {
			display: none;
		}
		
            `,
        ];
    }

    @internalProperty()
    active: boolean = false;

    onMouseEnter() {
        this.active = true;
    }

    onMouseLeave() {
        this.active = false;
    }

    render() {
        return html`
            <button
                @mouseenter="${this.onMouseEnter}"
                @mouseleave="${this.onMouseLeave}"
            >
                <img-ui class=${classMap({hidden: !this.active})} path="module/_common/play/play.svg"></img-ui>
                <img-ui class=${classMap({hidden: this.active})} path="module/_common/play/play-active.svg"></img-ui>
            </button>
        `;
    }
}
