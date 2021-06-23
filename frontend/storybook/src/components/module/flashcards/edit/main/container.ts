import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/module/flashcards/edit/main/container";
import "~/components/module/_groups/cards/edit/main/card-pair/card";
import {Mode} from "@elements/module/flashcards/edit/sidebar/option";
import {Card, Args as CardArgs} from "~/components/module/_groups/cards/edit/main/card-pair/card";
export default {
    title: "Module / Flashcards / Edit / Main" 
}

interface Args {
    mode: Mode,
}

const DEFAULT_ARGS:Args = {
    mode: "pair",
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {mode} = props;
    return `
    <flashcards-main>
	${mode === "single" ? renderSingle() : renderPair()}
    </flashcards-main>`;
}

/*
type CONTENT_MODE = "text" | "image" | "image-empty";
type IO_MODE = "edit" | "preview";

export interface Args {
    ioMode: IO_MODE,
    contentMode: CONTENT_MODE,
    editTarget: boolean,
    theme: ThemeKind,
    dragOver: boolean,
}
*/

function renderCard(inverted: boolean) {
	return Card({
		ioMode: "preview",
		contentMode: "image",
		editTarget: false,
		theme: "happy-brush",
		dragOver: false,
		size: "flashcards",
		inverted
	});
}
function renderSingle() {
	return `
		${renderCard(false)}
	`
}
function renderPair() {
	return `
		${renderCard(false)}
		${renderCard(true)}
	`
}
Container.args= DEFAULT_ARGS;

Container.argTypes = {
  mode: {
    control: {
      type: 'inline-radio',
      options: ["single", "pair"]
    }
  },
}