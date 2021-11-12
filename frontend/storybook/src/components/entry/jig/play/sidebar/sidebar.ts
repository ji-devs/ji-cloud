import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/sidebar";

export default {
    title: "Entry / Jig / Play / Sidebar",
};

interface Args {
    jigName: string;
    open: boolean;
}

const DEFAULT_ARGS: Args = {
    jigName: "Hebrew Letters - Tet",
    open: false,
};

export const Sidebar = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-sidebar ${argsToAttrs(props)}>
            <button-empty slot="close"><</button-empty>
            <button slot="opener"></button>
            <jig-play-sidebar-action slot="actions" kind="like"></jig-play-sidebar-action>
            <jig-play-sidebar-action slot="actions" kind="share"></jig-play-sidebar-action>
            <jig-play-sidebar-action slot="actions" kind="info"></jig-play-sidebar-action>
            <jig-sidebar-module slot="modules" module="cover" index="0">
                <img-ji slot="window" size="original" lib="mock" id="something.jpg"></img-ji>
            </jig-sidebar-module>
            <jig-sidebar-module slot="modules" module="flashcards" index="1">
                <img-ji slot="window" size="original" lib="mock" id="something.jpg"></img-ji>
            </jig-sidebar-module>
            <jig-sidebar-module slot="modules" module="matching" index="2">
                <img-ji slot="window" size="original" lib="mock" id="something.jpg"></img-ji>
            </jig-sidebar-module>
            <jig-sidebar-module slot="modules" module="poster" index="3">
                <img-ji slot="window" size="original" lib="mock" id="something.jpg"></img-ji>
            </jig-sidebar-module>
        </jig-play-sidebar>
    `;
};
Sidebar.args = DEFAULT_ARGS;
