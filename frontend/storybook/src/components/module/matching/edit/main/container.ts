import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/module-page/grid-resize";
import "@elements/module/matching/_common/main/container";
import "@elements/module/matching/_common/main/column";
import {Card, Args as CardArgs} from "~/components/module/_groups/cards/play/card";
import {Empty, Args as EmptyArgs} from "~/components/module/_groups/cards/play/empty";
import {Kind as EmptyKind} from "@elements/module/_groups/cards/play/card/empty";
import {StyleKind} from "@elements/module/_groups/cards/helpers";

export default {
    title: "Module / Matching / Edit / Main" 
}

interface Args {
    nPairs: number,
    colorBackground: boolean,
}

const DEFAULT_ARGS:Args = {
    nPairs: 4,
    colorBackground: true,
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {nPairs, colorBackground} = props;
    const style = colorBackground ? `style="background-color: #83cb6f;"` : "";

    return `

      <module-page-grid-resize>
        <matching-main slot="main" ${argsToAttrs(props)} ${style} >
          ${mapToString(arrayCount(nPairs), idx => {
            return renderTop(idx);
          })}
          ${mapToString(arrayCount(nPairs), idx => {
            return renderBottom(idx);
          })}
          ${renderDrag()}
        </matching-main>
      </module-page-grid-resize>
    `;
}

function renderTop(idx:number) {
  return `
    <matching-column slot="top">
      ${renderCard()}
      ${renderEmpty("question", idx === 2)}
    </matching-column>
  `;
}

function renderBottom(idx:number) {
  if(idx == 2) {
      return renderEmpty("translucent", false, "bottom");
  } else {
    return renderCard("bottom", "none");
  }
}

function renderDrag() {
	return Card({
		contentMode: "image",
		theme: "happy-brush",
		size: "matching",
		flipped: true,
		flipOnHover: false,
    slot: "drag",
    styleKind: "dragging",
    transform: "translate(350rem, 500rem)"
	});
}

function renderEmpty(kind: EmptyKind, active?: boolean, slot?:string) {
	return Empty({
		theme: "happy-brush",
		size: "matching",
    kind,
    active,
    slot
	});
}
function renderCard(slot?:string, styleKind?: StyleKind) {
	return Card({
		contentMode: "image",
		theme: "happy-brush",
		size: "matching",
		flipped: true,
		flipOnHover: false,
    slot,
    styleKind,
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