import "@elements/entry/jig/gallery/recent";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Entry / Jig / Gallery"
}

interface Args {
    draft: boolean;
    label: string;
    img: string;
    ages: string;
    lastEdited: string;
    href: string
}

const DEFAULT_ARGS:Args = {
    draft: false,
    label: "Teach New Vocabulary",
    img: "mock/resized/jig-gallery.jpg",
    ages: "5-8",
    lastEdited: "3 W ago",
    href: "https://google.com",
}

export const Recent = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <div style="padding: 30px;">
            <jig-gallery-recent ${argsToAttrs(props)}>
                <menu-line slot="menu-content" icon="delete"></menu-line>
            </jig-gallery-recent>
        </div>
    `;
}

Recent.args = DEFAULT_ARGS;

