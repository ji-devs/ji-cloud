import "@elements/core/images/ji";
import {MediaLibOptions, MediaSizeOptions} from "@utils/path";
import {injectSlotStr} from "@utils/slot";

export default {
  title: 'Core / Images',
}

/*** Ji - mock ****/

interface Args {
  size: MediaSizeOptions,
  slot?: string,
}

const DEFAULT_ARGS:Args = {
  size: "thumb"
}

export const Ji = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
  const {size, slot, slotStr} = injectSlotStr(props);
  //return `<img-ji lib="mock" size="${size}" id="image.png" ${slotStr}></img-ji>`
  return `<img-ji lib="mock" size="full" id="tall.png" ${slotStr}></img-ji>`
}

Ji.argTypes = {
  size: {
    control: {
      type: 'inline-radio',
      options: ["original", "full", "thumb"]
    }
  }
}


Ji.args = DEFAULT_ARGS; 
