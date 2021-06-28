import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/module-page/grid-resize";
import "@elements/module/flashcards/_common/main/container";
import "~/components/module/_groups/cards/edit/main/card-pair/card";
import {Mode} from "@elements/module/flashcards/edit/sidebar/option";
import {Card, Args as CardArgs} from "~/components/module/_groups/cards/play/card";
export default {
    title: "Module / Flashcards / Edit / Main" 
}

interface Args {
    mode: Mode,
}

const DEFAULT_ARGS:Args = {
    mode: "single",
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {mode} = props;
    return `

      <module-page-grid-resize>
	<flashcards-main slot="main">
		${mode === "single" ? renderSingle() : renderPair()}
	</flashcards-main>
      </module-page-grid-resize>
      `;
}

interface CardOptions {
	flipped: boolean,
	flippable: boolean,
	doubleSided: boolean,
}
function renderCard({flipped, flippable, doubleSided}) {
	const cardArgs:Partial<CardArgs> = {
		contentMode: "image",
		theme: "happy-brush",
		size: "flashcards",
		flipped,
		flipOnHover: flippable,
	};

	if(doubleSided) {
		cardArgs.backSideContent = "text";
	}

	return Card(cardArgs);
}
function renderSingle() {
	return `
		${renderCard({
			flipped: true,
			flippable: true,
			doubleSided: true
		})}
	`
}
function renderPair() {
	return `
		${renderCard({
			flipped: true,
			flippable: false,
			doubleSided: false
		})}
		${renderCard({
			flipped: false,
			flippable: true,
			doubleSided: false
		})}
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