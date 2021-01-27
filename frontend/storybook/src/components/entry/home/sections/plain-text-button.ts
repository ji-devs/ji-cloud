import "@elements/core/buttons/text";
// import "@elements/buttons/replace-delete";

export default {
  title: 'Entry /Home/Widgets/Plain Text Button',}

export const PlainTextButton = ({label, color, size, bold, italic}) => {
    return `<button-text label="${label}" color="${color}" size="${size}" ${bold && "bold"} ${italic && 'italic'}/>`
}





PlainTextButton.args = {
  label: "Replace",
  color: 'blue',
  size: 'small',
  bold: false,
  italic: false,
}


