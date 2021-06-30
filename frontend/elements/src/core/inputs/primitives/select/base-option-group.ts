import { LitElement, html, css, customElement, property, internalProperty, PropertyValues, query } from "lit-element";
import { OptionItem, OptionsContainer } from "./options-interfaces";
import { BaseOption } from "./base-option";
import { OptionItemMixin } from "./option-item-mixin";
import { OptionsCollectionMixin } from "./options-collection-mixin";
import "@elements/core/overlays/anchored-overlay";

@customElement("base-option-group")
export class BaseOptionGroup extends OptionItemMixin(OptionsCollectionMixin(LitElement)) implements OptionItem, OptionsContainer {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                }
                anchored-overlay {
                    display: block;
                }
                .anchor {
                    cursor: pointer;
                }
            `,
        ];
    }

    multiple = false;

    @property({ type: Boolean, reflect: true, attribute: "selected-within" })
    selectedWithin: boolean = false;

    firstUpdated(){
        super.firstUpdated();

        if(this.container) {
            this.multiple = this.container.multiple;
        }
    }

    optionSelected(option: BaseOption): void {
        this.selectedWithin = true;
        if(this.container) {
            this.container.optionSelected(option);
        }
    }
    optionDeselected(): void {
        for (const option of this.options as Array<BaseOptionGroup | BaseOption>) {
            // @ts-ignore
            if(option.selected || option.selectedWithin) {
                this.selectedWithin = true; // probably not needed, since being here means that selected-within is already set to true
                return;
            }
        }
        this.selectedWithin = false;
        if(this.container) {
            this.container.optionDeselected();
        }
    }
    optionActivated(depth: number): void {
        if(this.container) {
            this.container.optionActivated(depth + 1);
        }
        this.activeWithin = depth;
    }
    optionDeactivate(): void {
        if(this.container) {
            this.container.optionDeactivate();
        }
        this.activeWithin = undefined;
    }

    private onAnchorClick() {
        this.open = true;
        if(this.active)
            this.setInactive();
        else
            this.setActive();
    }

    @internalProperty()
    open: boolean = false;

    @query("[slot=overlay]")
    overlay!: HTMLSlotElement;

    render() {
        return html`
            <anchored-overlay
                ?open="${this.open}"
                positionX="right-out"
                positionY="top-in"
                .autoClose="${false}"
                @close="${() => this.open = false}"
                exportparts="overlay"
            >
                <div class="anchor" slot="anchor" @click="${this.onAnchorClick}">
                    <slot name="anchor"></slot>
                </div>
                <slot slot="overlay"></slot>
            </anchored-overlay>
        `;
    }
}

