import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/selection/card";
import {mapToString, arrayIndex} from "@utils/array";
import {ModuleKind, moduleKinds} from "@elements/entry/jig/module-types";

export default {
    title: "Entry / Jig / Edit / Selection"
}

interface Args {
    module: ModuleKind,
    hover: boolean,
    drag: boolean,
}

const DEFAULT_ARGS:Args = {
    module: "memory",
    hover: false,
    drag: false,
}

export const Card = (props?:Partial<Args> & {slot?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {slot, ...cardProps} = props;

    return `<jig-edit-module-card ${argsToAttrs(cardProps)} ${slot && `slot="${slot}"`}></jig-edit-module-card>
    `;
}

Card.args = DEFAULT_ARGS;


Card.argTypes = {
  module: {
    control: {
      type: 'inline-radio',
      options: moduleKinds.filter(module => module !== "cover") 
    }
  }
}
