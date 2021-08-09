import { LitElement, html, css, customElement, property, query } from "lit-element";
import { BaseOption } from "./base-option";
import { BaseOptionGroup } from "./base-option-group";
import { OptionsContainer } from "./options-interfaces";
import { OptionsCollectionMixin } from "./options-collection-mixin";
import "@elements/core/overlays/anchored-overlay";


@customElement("base-select")
export class BaseSelect extends OptionsCollectionMixin(LitElement) implements OptionsContainer {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                }
                :host(:focus) {
                    outline: 0;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    open: boolean = false;

    @property({ type: Boolean })
    multiple: boolean = false;

    @query("[slot=overlay]")
    overlay!: HTMLSlotElement;

    private buttonPress(e: KeyboardEvent) {
        switch (e.code) {
            case "ArrowDown":
                this.down();
                break;
            case "ArrowUp":
                this.up();
                break;
            case "Space":
            case "Enter":
                this.toggleSelected();
                break;
            case "Escape":
                this.close();
                break;
        }
    }

    connectedCallback() {
        super.connectedCallback();
        super.tabIndex = 0;
        this.setAttribute("role", "listbox");
        this.addEventListener("keydown", this.buttonPress);
    }

    optionSelected(option: BaseOption): void {
        this.dispatchEvent(new Event("option-selected"));

        if(!this.multiple) {
            this.close();

            this.deselectAll(option);
        }
    }

    optionDeselected(): void {
        this.dispatchEvent(new Event("option-deselected"));
    }

    toggleSelected() {
        if(this.activeWithin === undefined) {
            // and when closed
            this.toggleOpen();
        } else {
            super.toggleSelected();
        }
    }

    optionActivated(depth: number): void {
        this.clearAllActive(this.options as any[]);
        this.activeWithin = depth;
    }

    optionDeactivate(): void {
        this.activeWithin = undefined;
    }

    private close() {
        this.open = false;
        this.clearAllActive(this.options);
        this.triggerOpenChangeEvent();
    }

    private openUp() {
        this.open = true;
        this.triggerOpenChangeEvent();
    }

    private toggleOpen() {
        if(this.open) {
            this.close();
        } else {
            this.openUp();
        }
    }

    private triggerOpenChangeEvent() {
        this.dispatchEvent(new CustomEvent("custom-open-change", {
            bubbles: true, // should be here?
            detail: {
                open: this.open
            }
        }));
    }

    private clearAllActive(container: Array<BaseOptionGroup | BaseOption>) {
        for (const option of container) {
            if(option.active) {
                option.setInactive();
                return; // since only one can be active at a time, we can return after finding one active
            } else if (option instanceof BaseOptionGroup && option.activeWithin !== undefined) {
                this.clearAllActive(option.options as any[]);
            }
        }
    }

    render() {
        return html`
            <anchored-overlay ?open="${this.open}" .autoClose="${false}" @close="${this.close}" positionX="left-in" part="anchored-overlay" exportparts="overlay: anchored-overlay_overlay">
                <slot slot="anchor" name="anchor" @click="${this.toggleOpen}"></slot>
                <slot slot="overlay"></slot>
            </anchored-overlay>
        `;
    }
}

export type OptOrGrp = BaseOption | BaseOptionGroup;
