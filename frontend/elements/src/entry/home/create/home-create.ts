import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/buttons/rectangle";
import { homeStyles } from '../styles';

@customElement('home-create')
export class _ extends LitElement {
    static get styles() {
        return [homeStyles, css`
            :host {
                background-color: var(--yellow-1);
                display: block;
            }
            .width-holder {
                padding: 0;

                display: grid;
                grid-template-columns: 900px auto;
            }
            .video-wrapper {
                background-color: #fee595;
            }
            .content {
                padding: 72px;
                display: grid;
                grid-template-rows: auto auto 1fr auto;
            }
            h3 {
                font-size: 32px;
                font-weight: 300;
                color: #383838;
                margin: 0;
            }
            h2 {
                margin: 0;
                font-size: 64px;
                font-weight: 900;
                color: var(--dark-blue-4);
            }
            h2 .word-create {
                color: #fd7076;
            }
            ul {
                padding-inline-start: 20px;
                margin: 48px 0;
                align-self: center;
            }
            li::marker {
                color: #fed758;
            }
        `];
    }

    render() {
        const STR_SUBTITLE = "Learning Through Creation";
        const STR_TITLE_CREATE = "Create";
        const STR_TITLE = "your own JIGs";

        const STR_LIBRARY = "Big content library";
        const STR_INTERFACE = "Smart & friendly interface";
        const STR_TEACHING = "Teaching through creation";
        const STR_ALL = "All in one";
        const STR_START ="Start creating";

        return html`
            <div class="width-holder">
                <div class="video-wrapper">
                    
                </div>
                <div class="content">
                    <h3>${STR_SUBTITLE}</h3>
                    <h2>
                        <span class="word-create">${STR_TITLE_CREATE}</span>
                        ${STR_TITLE}
                    </h2>
                    <ul>
                        <li>${STR_LIBRARY}</li>
                        <li>${STR_INTERFACE}</li>
                        <li>${STR_TEACHING}</li>
                        <li>${STR_ALL}</li>
                    </ul>
                    <button-rect color="red">${STR_START}</button-rect>
                </div>
            </div>
        `;
    }
}