import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('social-networks')
export class _ extends LitElement {
    static get styles() {
        return [css`
            img-ui {
                width: 32px;
                height: 32px;
                margin-right:24px;
                display:block;
            }
            .wrapper {
                display:flex;
            }
        `]
    }

    @property()
    path_facebook: string = "";
    @property()
    path_instagram: string = "";
    @property()
    path_youtube: string = "";
    @property()
    path_linkedin: string = "";

    render() {
        const { path_facebook, path_instagram, path_youtube, path_linkedin } = this;

        return html`
            <div class="wrapper">
                <img-ui class="img" path="${path_facebook}"></img-ui>
                <img-ui class="img" path="${path_instagram}"></img-ui>
                <img-ui class="img" path="${path_youtube}"></img-ui>
                <img-ui class="img" path="${path_linkedin}"></img-ui>
            </div>
        `;
    }
}
