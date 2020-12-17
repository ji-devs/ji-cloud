import "@elements/inputs/input-text";
import "@elements/buttons/rectangle-button";
import {RectangleButtonBlueMedium} from "~/components/rectangle-button";
export default {
  title: 'Input Text',
}

export const InputTextIdle = () => {
    return `<input-text label="Title" helpertext="Minimum 8 digits, Must include a number" error="Wrong Password">
 
    </input-text>`
}

