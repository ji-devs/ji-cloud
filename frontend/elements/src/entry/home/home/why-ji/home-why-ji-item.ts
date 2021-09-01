import { LitElement, html, css, customElement, property } from 'lit-element';

export type Kind = 'content' | 'create' | 'customize' | 'community' | 'classroom';

const STR_CONTENT_TITLE = "Content";
const STR_CONTENT_PARAGRAPH = "A huge library of activities and lessons for the Jewish holidays, Hebrew, Israel, Torah and many more topics.";
// const STR_CONTENT_ACTION = "See our templates";

const STR_CREATE_TITLE = "Create";
const STR_CREATE_PARAGRAPH = "Create your own interactive lessons and games (JIGs), teach your class to create their own games. The most fun way to learn something new.";
// const STR_CREATE_ACTION = "Try it for free";

const STR_CUSTOMIZE_TITLE = "Customize";
const STR_CUSTOMIZE_PARAGRAPH = "It’s easy and fast! Customize our ready-made lesson outlines for your needs.";
// const STR_CUSTOMIZE_ACTION = "See our templates";

const STR_COMMUNITY_TITLE = "Community";
const STR_COMMUNITY_PARAGRAPH = "Join the Jigzi community to meet educators from around the world and share your thoughts and creativity.";
// const STR_COMMUNITY_ACTION = "Get inspired";

const STR_CLASSROOM_TITLE = "Classroom";
const STR_CLASSROOM_PARAGRAPH = "Create classes, assign JIGs to students, then track your students’ journeys.";
// const STR_CLASSROOM_ACTION = "Manage your class";

interface KindDetails {
    paragraph: string,
    title: string,
}

const STR_DETAILS_LOOKUP: {
    [key in Kind]: KindDetails
} = {
    ['content']: {
        paragraph: STR_CONTENT_PARAGRAPH,
        title: STR_CONTENT_TITLE
    },
    ['create']: {
        paragraph: STR_CREATE_PARAGRAPH,
        title: STR_CREATE_TITLE
    },
    ['customize']: {
        paragraph: STR_CUSTOMIZE_PARAGRAPH,
        title: STR_CUSTOMIZE_TITLE
    },
    ['community']: {
        paragraph: STR_COMMUNITY_PARAGRAPH,
        title: STR_COMMUNITY_TITLE
    },
    ['classroom']: {
        paragraph: STR_CLASSROOM_PARAGRAPH,
        title: STR_CLASSROOM_TITLE
    },
};

@customElement('home-why-ji-item')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                width: 274px;
                display: grid;
                grid-template-rows: auto auto 1fr auto;
                row-gap: 16px;
            }
            img-ui {
                width: 100%;
                object-fit: contain;
            }
            h4 {
                font-size: 32px;
                font-weight: 900;
                margin: 0;
            }
            :host([kind=content]) h4 {
                color: #fd6b71;
            }
            :host([kind=create]) h4 {
                color: #2040a3;
            }
            :host([kind=customize]) h4 {
                color: #46ba6f;
            }
            :host([kind=community]) h4 {
                color: #fea559;
            }
            :host([kind=classroom]) h4 {
                color: #6ca1fc;
            }
            p {
                font-family: Poppins;
                line-height: 1.5;
                color: #383838;
                margin-top: 10px;
                margin: 0;
            }
        `]
    }

    @property({reflect: true})
    kind: Kind = "classroom";

    render() {
        let kindDetails = STR_DETAILS_LOOKUP[this.kind];

        return html`
            <img-ui class="img" path="entry/home/why-ji/${this.kind}.png"></img-ui>
            <h4>${kindDetails.title}</h4>
            <p>${kindDetails.paragraph}</p>
            <slot></slot>
        `;
    }
}
