import { LitElement, html, css, customElement, property } from 'lit-element';

const STR_LABEL = "Put each item where it should be dropped.";

@customElement('module-sidebar-drag-prompt')
export class _ extends LitElement {
    static get styles() {
        return [css`
		:host {
			width: 100%;
			height: 100%;
			display: flex;
			flex-direction: column;
			justify-content: center;
			align-items: center;
		}

		.label {
			font-size: 18px;
			font-weight: 500;
			color: var(--dark-gray-6);
		}
        `];
    }

    render() {
        return html`
		<img-ui path="module/_common/edit/widgets/sidebar/drag-prompt.svg"></img-ui>
		<div class="label">${STR_LABEL}</div>
        `;
    }
}
