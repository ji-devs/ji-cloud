import "@elements/buttons/circle-button";
export default {
  title: 'Circle Button',
}

export const CircleButton = ({text, label, active, disabled}) => {
    return `<circle-button text="${text}" label="${label}" ${active && "active"} ${disabled && "disabled"}/>`
}

CircleButton.args = {
    text: "1",
    label: "Label",
    active: true,
    disabled: false,
}