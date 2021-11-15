// pattern from https://www.typescriptlang.org/docs/handbook/mixins.html

import { closestPierceSlot } from "@utils/dom";
import { LitElement, property } from "lit-element";

type Constructor = new (...args: any[]) => {};
// type GConstructor<T = {}> = new (...args: any[]) => T;
// type Spritable = GConstructor<LitElement>;

export function OptionItemMixin<TBase extends Constructor>(Base: TBase) {
    class OptionItem extends Base {
        container: any | null;

        @property({ type: Boolean, reflect: true })
        active: boolean = false;

        setActive() {
            // first have the container clear all actives and then set active to true
            if (this.container) {
                this.container.optionActivated(1);
            }

            this.active = true;
        }

        setInactive() {
            if (this.container) {
                this.container.optionDeactivate();
            }
            this.active = false;
        }

        firstUpdated() {
            this.container = closestPierceSlot(
                // @ts-ignore
                this.assignedSlot || this.parentNode,
                "base-select, base-option-group"
            ) as any | null;

            if (this.container) {
                this.container.registerOption(this);
            }
        }

        disconnectedCallback() {
            // @ts-ignore
            super.disconnectedCallback();
            if (this.container) {
                this.container.unregisterOption(this);
            }
        }
    }
    return OptionItem;
}
