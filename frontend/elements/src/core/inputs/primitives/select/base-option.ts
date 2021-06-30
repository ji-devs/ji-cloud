import { LitElement, html, css, customElement, property } from "lit-element";
import { OptionItem } from "./options-interfaces";
import { OptionItemMixin } from "./option-item-mixin";

@customElement("base-option")
export class BaseOption extends OptionItemMixin(LitElement) implements OptionItem {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    cursor: pointer;
                }
            `,
        ];
    }

    // maybe change this to a getter?
    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    public toggleSelected() {
        // deselection only allowed when in multiple mode
        if(!this.selected || this.container?.multiple) {
            if(this.container) {
                if(!this.selected) {
                    this.container.optionSelected();
                    this.selected = true;
                } else {
                    this.container.optionDeselected();
                    this.selected = false;
                }
                
            }
            this.dispatchSelectedChange();
        }
    }

    public _deselectingAll() {
        // when deselecting all don's call container.optionSelected/Deselected
        this.selected = false;
        this.dispatchSelectedChange();
    }

    private dispatchSelectedChange() {
        this.dispatchEvent(new CustomEvent("custom-selected", {
            bubbles: true, // should be here?
            detail: {
                selected: this.selected
            }
        }));
    }

    connectedCallback() {
        super.connectedCallback();

        this.addEventListener("click", this.onClick);
        this.setAttribute("role", "option");
    }

    onClick() {
        this.setActive();
        this.toggleSelected();
    }

    render() {
        return html`
            <slot></slot>
        `;
    }
}
