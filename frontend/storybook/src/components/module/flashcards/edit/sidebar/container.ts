import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/module/flashcards/edit/sidebar/container";
import "@elements/module/flashcards/edit/sidebar/option";
import {Mode} from "@elements/module/flashcards/edit/sidebar/option";

export default {
    title: "Module / Flashcards / Edit / Sidebar" 
}

interface Args {
    selected: Mode,
}

const DEFAULT_ARGS:Args = {
    selected: "single",
}

const modes:Array<Mode> = ["single", "double"];

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {selected} = props;
    return `<flashcards-settings>
        ${mapToString(modes, opt_mode => 
            `<flashcards-settings-option ${opt_mode === selected ? "selected" : ""} mode="${opt_mode}"></flashcards-settings-option>`
        )}
    </flashcards-settings>`;
}

Container.args= DEFAULT_ARGS;

Container.argTypes = {
  selected: {
    control: {
      type: 'inline-radio',
      options: modes 
    }
  },
}