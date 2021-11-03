import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/buttons/rectangle";
import { homeStyles } from '../styles';
import { mediaUi } from '@utils/path';

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
                grid-template-columns:  minmax(100px, 1000px) auto;
            }
            .video-wrapper {
                display: grid;
                align-items: center;
            }
            .video-wrapper video {
                object-position: center center;
                width: 100%;
            }
            .content {
                padding: 52px;
                display: grid;
                grid-template-rows: auto auto 1fr auto;
                justify-items: start;
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
            h4 {
                margin: 0;
                font-weight: 400;
                font-size: 20px;
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
        const STR_JIG_EXPLAINER = "Create your own Jewish Interactive Games (JIGs) for your class in minutes!";

        const STR_LIBRARY = "Large library of content";
        const STR_INTERFACE = "Smart, friendly interface";
        const STR_TEACHING = "Teaching through creation and engagement";
        const STR_START ="Start creating";

        return html`
            <div class="width-holder">
                <div class="video-wrapper">
                    <video
                        controls
                        autoplay
                        muted
                        src=${mediaUi('entry/home/create/video.mp4')}
                    ></video>
                </div>
                <div class="content">
                    <h3>${STR_SUBTITLE}</h3>
                    <h2>
                        <span class="word-create">${STR_TITLE_CREATE}</span>
                        ${STR_TITLE}
                    </h2>
                    <h4>${STR_JIG_EXPLAINER}</h4>
                    <ul>
                        <li>${STR_LIBRARY}</li>
                        <li>${STR_INTERFACE}</li>
                        <li>${STR_TEACHING}</li>
                    </ul>
                    <button-rect color="red" href="/jig/edit/gallery">${STR_START}</button-rect>
                </div>
            </div>
        `;
    }
}