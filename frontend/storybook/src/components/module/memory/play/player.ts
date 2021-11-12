import { argsToAttrs } from "@utils/attributes";
import "@elements/module/memory/play/container";
import "@elements/core/module-page/grid-resize";

import { Sidebar } from "./sidebar";
import { Main } from "./main";
import { Ending } from "./ending";
import { mapToString, arrayIndex } from "@utils/array";

export default {
    title: "Module / Memory / Play",
};

interface Args {
    nCards: number;
    isEnding: boolean;
}

const DEFAULT_ARGS: Args = {
    nCards: 6,
    isEnding: false,
};

export const Player = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { nCards, isEnding } = props;

    return `

      <module-page-grid-resize>
    <memory-container slot="main">
        ${Sidebar({ nPairs: nCards / 2 })}
        ${isEnding ? Ending() : Main({ nCards })}
    </memory-container>
      </module-page-grid-resize>
    `;
};

Player.args = DEFAULT_ARGS;
