import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import { nothing } from "lit-html";
import "@elements/core/images/ui";

export type TitleKind = ""
    | 'theme'
    | 'background-image'
    | 'background-color'
    | 'color'
    | 'overlay'
    | 'text'
    | 'image'
    | 'audio'
    | 'select'
    | 'play-settings'
    | 'instructions';

const STR_LABEL_LOOKUP: {
    [key in TitleKind]: string;
} = {
    ['']: '',
    ['select']: 'Select',
    ['theme']: 'Theme',
    ['background-image']: 'Background image',
    ['background-color']: 'Background color',
    ['color']: 'Color',
    ['overlay']: 'Overlay',
    ['text']: 'Text',
    ['image']: 'Image',
    ['audio']: 'Audio',
    ['play-settings']: 'Play Settings',
    ['instructions']: 'Instructions',
};

const getIcon = (kind:TitleKind):TitleKind => {
    if(kind === "background-color") {
        return("color");
    } else {
        return(kind);
    }
}

@customElement("menu-tab-title")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        :host {
          display: flex;
          font-family: Poppins;
          font-size: 16px;
          font-weight: 500;
        }

        .highlight {
          color: var(--main-blue);
        }

        img-ui {
          max-width: 24px;
          max-height: 24px;
          margin-right: 8px;
          display: flex;
        }

        .hidden {
          display: none;
        }
      `,
    ];
  }

  onEnter() {
    this.hover = true;
  }

  onLeave() {
    this.hover = false;
  }

  @property({ type: Boolean })
  hover: boolean = false;

  @property()
  kind: TitleKind = "";

  @property({ type: Boolean, reflect: true })
  active: boolean = false;

  connectedCallback() {
    super.connectedCallback();

    this.addEventListener("mouseenter", this.onEnter);
    this.addEventListener("mouseleave", this.onLeave);
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    this.removeEventListener("mouseenter", this.onEnter);
    this.removeEventListener("mouseleave", this.onLeave);
  }

  render() {
    const { kind, active, hover } = this;

    const highlight = active || hover;

    const label = STR_LABEL_LOOKUP[this.kind];
    const iconUrl = `module/_common/edit/widgets/sidebar/menu-tabs/${getIcon(kind)}.svg`;
    const iconUrlActive = `module/_common/edit/widgets/sidebar/menu-tabs/${getIcon(kind)}-active.svg`;

    const regularClass = classMap({ hidden: highlight });
    const activeClass = classMap({ hidden: !highlight });

    const labelClass = classMap({ highlight });
    return html`
      ${this.kind === ""
        ? nothing
        : html`
            <img-ui class=${regularClass} path="${iconUrl}"></img-ui>
            <img-ui class=${activeClass} path="${iconUrlActive}"></img-ui>
          `}
      <div class=${labelClass}>${label}</div>
    `;
  }
}
