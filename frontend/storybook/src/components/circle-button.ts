import "@elements/buttons/circle-button";
export default {
  title: 'Circle Button',
}

export const CircleButtonActive = () => {
    return `<circle-button text="1" label="Active" active />`
}

export const CircleButtonInactive = () => {
    return `<circle-button text="1" label="Inactive" />`
}

export const CircleButtonDisabled = () => {
    return `<circle-button text="1" label="Disabled (TODO)" disabled />`
}

