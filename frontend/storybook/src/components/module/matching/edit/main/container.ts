import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/module-page/grid-resize";
import "@elements/module/card-quiz/edit/main/container";
import "~/components/module/_groups/cards/edit/main/card-pair/card";
import {Card, Args as CardArgs} from "~/components/module/_groups/cards/play/card";
export default {
    title: "Module / Matching / Edit / Main" 
}

interface Args {
    nPairs: number,
}

const DEFAULT_ARGS:Args = {
    nPairs: 4,
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {nPairs} = props;
    return `

      <module-page-grid-resize>
    <card-quiz-main slot="main" ${argsToAttrs(props)}>
	${mapToString(arrayCount(nPairs), idx => {
		return renderCard(false);
	})}
    </card-quiz-main>
      </module-page-grid-resize>
    `;
}

function renderCard(flipped: boolean) {
	return Card({
		contentMode: "image",
		theme: "happy-brush",
		size: "flashcards",
		flipped,
		flipOnHover: false 
	});
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