import { LitElement, html, css, customElement, property } from 'lit-element';

export type Kind = 'home' | 'content' | 'create' | 'community' | 'classroom' | 'about';

const STR_LABEL_LOOKUP: {
    [key in Kind]: string
} = {
    ['home']: 'Home',
    ['content']: 'Content',
    ['create']: 'Create',
    ['community']: 'Community',
    ['classroom']: 'Classroom',
    ['about']: 'About JI',
};

@customElement('page-header-link')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                box-sizing: border-box;
                border-top: solid 6px transparent;
                display: grid;
                grid-template-columns: auto auto;
                column-gap: 6px;
                margin: 0 26px;
                align-items: center;
                justify-content: space-between;
                cursor: pointer;
                font-weight: 500;
            }
            :host([active]) {
                border-color: #fd7076;
                color: var(--dark-red-1);
            }
        `];
    }

    @property()
    kind: Kind = "home";

    @property({ type: Boolean })
    active: boolean = false;

    render() {
        const path = `core/page-header/nav-icon-${this.kind}${this.active ? '-active' : ''}.svg`;
        return html`
            <img-ui path="${path}"></img-ui>
            <span>${STR_LABEL_LOOKUP[this.kind]}</span>
        `;
    }
}
