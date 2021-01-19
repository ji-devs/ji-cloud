import "@elements/core/images/ui";
import {MediaLibOptions, MediaSizeOptions} from "@utils/path";
import {injectSlotStr} from "@utils/slot";

export default {
  title: 'Core / Images',
}

/*** Ui ****/
interface Args {
  path: string,
  slot?: string,
}

const DEFAULT_ARGS:Args = {
  path: `sidebar-logo.png`
}

export const Ui = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {path, slotStr} = injectSlotStr(props);
    return `<img-ui path="${path}" ${slotStr}></img-ui>`
}

Ui.args = DEFAULT_ARGS;