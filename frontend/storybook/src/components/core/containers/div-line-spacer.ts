import "@elements/core/containers/div-line-spacer";
import "@elements/core/buttons/text";
import { arrayCount, mapToString } from "@utils/array";

export default {
  title: 'Core / Containers',
}

interface Args {
  nSlots: number
}

const DEFAULT_ARGS:Args = {
  nSlots: 3
}

export const DivLineSpacer = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nSlots} = props;

    return `<div-line-spacer nSlots="${nSlots}" >
        ${mapToString(arrayCount(nSlots), i => 
            `<button-text slot="slot-${i}" color="blue" size="none">Slot ${i}</button-text>`
        )}
    </div-line-spacer>`;
}

DivLineSpacer.args = DEFAULT_ARGS;