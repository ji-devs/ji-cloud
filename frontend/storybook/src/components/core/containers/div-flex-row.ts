import "@elements/core/containers/div-flex-row";
import "@elements/core/buttons/rectangle";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Core / Containers",
};

interface Args {
    nSlots: number;
}

const DEFAULT_ARGS: Args = {
    nSlots: 3,
};

export const DivFlexRow = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { nSlots } = props;

    return `<div-flex-row>
        ${mapToString(
            arrayCount(nSlots),
            (i) =>
                `<button-rect kind="text" color="blue" size="none">Slot ${i}</button-rect>`
        )}
    </div-flex-row>`;
};

DivFlexRow.args = DEFAULT_ARGS;
