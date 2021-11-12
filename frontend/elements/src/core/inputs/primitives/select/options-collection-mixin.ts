// pattern from https://www.typescriptlang.org/docs/handbook/mixins.html

import { property } from "lit-element";
import { BaseOption } from "./base-option";
import { BaseOptionGroup } from "./base-option-group";

type Constructor = new (...args: any[]) => {};

export function OptionsCollectionMixin<TBase extends Constructor>(Base: TBase) {
    class OptionsCollection extends Base {
        @property({
            reflect: true,
            attribute: "active-within",
            converter: {
                fromAttribute: () => {
                    throw new Error("Can't be set from attribute");
                },
                toAttribute: (num: number) => {
                    // empty string sets the attribute and null removes it
                    return num ? "" : null;
                },
            },
        })
        activeWithin?: number;

        options: Array<BaseOption | BaseOptionGroup> = [];

        registerOption(option: any) {
            this.options.push(option); // TODO:, how will I know the index??
        }
        unregisterOption(_option: any): void {}

        up() {
            const options = this.options;

            for (let i = 1; i < options.length; i++) {
                if (options[i].active) {
                    options[i].setInactive();
                    options[i - 1].setActive();
                    return;
                    //@ts-ignore
                } else if (options[i].activeWithin) {
                    (options[i] as BaseOptionGroup).up();
                    return;
                }
            }

            // if last item isn't selected
            if (options.length && !options[0].active) {
                options[options.length - 1].active = true;
            }
        }

        down() {
            const options = this.options;

            for (let i = 0; i < options.length - 1; i++) {
                if (options[i].active) {
                    options[i].setInactive();
                    options[i + 1].setActive();
                    return;
                    //@ts-ignore
                } else if (options[i].activeWithin) {
                    (options[i] as BaseOptionGroup).down();
                    return;
                }
            }

            // if first item isn't selected
            if (options.length && !options[options.length - 1].active) {
                options[0].active = true;
            }
        }

        toggleSelected() {
            for (const option of this.options) {
                if (option instanceof BaseOption && option.active) {
                    option.toggleSelected();
                    return;
                } else if (option instanceof BaseOptionGroup) {
                    if (option.active) option.open = !option.open;
                    else if (option.activeWithin) option.toggleSelected();
                }
            }
        }

        deselectAll(except: BaseOption) {
            for (const option of this.options) {
                if (
                    option instanceof BaseOption &&
                    option.selected &&
                    option !== except
                ) {
                    option._deselectingAll();
                } else if (option instanceof BaseOptionGroup) {
                    if (option.selectedWithin) option.deselectAll(except);
                }
            }
        }
    }
    return OptionsCollection;
}
