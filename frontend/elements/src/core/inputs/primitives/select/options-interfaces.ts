// TODO:Change all properties to getters

export interface OptionsContainer {
    // depth index
    activeWithin?: number;

    options: OptionItem[];

    multiple: boolean;

    registerOption(option: OptionItem): void;
    unregisterOption(option: OptionItem): void;

    optionSelected(option: OptionItem): void;
    optionDeselected(): void;

    optionActivated(depth: number): void;
    optionDeactivate(): void;
}

export interface OptionItem {
    container: OptionsContainer | null;

    active: boolean;

    setActive(): void;

    setInactive(): void;
}
