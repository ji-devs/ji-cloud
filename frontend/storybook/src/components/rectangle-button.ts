import "@elements/buttons/rectangle-button";
export default {
  title: 'Rectangle Button',
}

export const RectangleButton = ({label, color, size, bold, italic}) => {
    return `<rectangle-button label="${label}" color="${color}" size="${size}" ${bold && "bold"} ${italic && 'italic'}/>`
}




RectangleButton.args = {
  label: "Placeholder",
  color: 'blue',
  size: 'medium',
  bold: true,
  italic: false,
}

