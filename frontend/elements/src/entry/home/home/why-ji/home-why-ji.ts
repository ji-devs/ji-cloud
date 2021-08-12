import { LitElement, html, css, customElement, property, unsafeCSS } from 'lit-element';
import "@elements/core/buttons/rectangle";
import "./home-why-ji-item";
import { mediaUi } from '@utils/path';
import { homeStyles } from '../styles';

@customElement('home-why-ji')
export class _ extends LitElement {
    static get styles() {
        return [homeStyles, css`
            :host {
                display: block;
                background-image: url("${unsafeCSS(mediaUi("entry/home/why-ji/background.png"))}");
                background-size: cover;
                padding: 72px 0;
            }
            h2 {
                font-size: 64px;
                font-weight: 900;
                color: var(--dark-blue-4);
                text-align: center;
                margin: 0;
            }
            .items-wrapper {
                display: flex;
                column-gap: 50px;
                justify-content: space-between;
            }
        `];
    }

    render() {
        const STR_CONTENT_ACTION = "See our templates";
        const STR_CREATE_ACTION = "Try it for free";
        const STR_CUSTOMIZE_ACTION = "See our templates";
        const STR_COMMUNITY_ACTION = "Get inspired";
        const STR_CLASSROOM_ACTION = "Manage your class";
        const STR_TITLE = "Why Ji?";

        return html`
            <div class="width-holder">
                <h2>${STR_TITLE}</h2>

                <div class="items-wrapper">
                    <home-why-ji-item kind="content">
                        <button-rect kind="text" color="blue" size="small" weight="normal">${STR_CONTENT_ACTION}</button-rect>
                    </home-why-ji-item>
                    <home-why-ji-item kind="create">
                        <button-rect kind="text" color="blue" size="small" weight="normal">${STR_CREATE_ACTION}</button-rect>
                    </home-why-ji-item>
                    <home-why-ji-item kind="customize">
                        <button-rect kind="text" color="blue" size="small" weight="normal">${STR_CUSTOMIZE_ACTION}</button-rect>
                    </home-why-ji-item>
                    <home-why-ji-item kind="community">
                        <button-rect kind="text" color="blue" size="small" weight="normal">${STR_COMMUNITY_ACTION}</button-rect>
                    </home-why-ji-item>
                    <home-why-ji-item kind="classroom">
                        <button-rect kind="text" color="blue" size="small" weight="normal">${STR_CLASSROOM_ACTION}</button-rect>
                    </home-why-ji-item>
                </div>
            </div>
        `;
    }
}
