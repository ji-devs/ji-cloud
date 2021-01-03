import "@elements/buttons/rectangle-button";
export default {
  title: 'Rectangle Button',
}

interface ButtonArgs {
  label: string,
  color: string,
  size: string,
  bold: boolean,
  italic: boolean

}

const DEFAULT_ARGS:ButtonArgs = {
  label: "Submit",
  color: "red",
  size: "medium",
  bold: false,
  italic: false,
}

export const RectangleButton = (props?:ButtonArgs) => {

  const {label, color, size, bold, italic} = props || DEFAULT_ARGS;
    return `<rectangle-button label="${label}" color="${color}" size="${size}" ${bold && "bold"} ${italic && 'italic'}/>`
}




RectangleButton.args = DEFAULT_ARGS;
