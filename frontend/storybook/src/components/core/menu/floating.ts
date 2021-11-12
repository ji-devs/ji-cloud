import { argsToAttrs } from "@utils/attributes";
import { mapToString } from "@utils/array";
import "@elements/core/drag/container";
import "@elements/core/menu/container";
import "@elements/core/menu/menu-line";

import { Container as DragContainer } from "~/components/core/drag/container";
import { Container as MenuContainer } from "~/components/core/menu/container";

export default {
    title: "Core / Menu",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Floating = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return DragContainer({ childHtml: MenuContainer() });
};

Floating.args = DEFAULT_ARGS;
