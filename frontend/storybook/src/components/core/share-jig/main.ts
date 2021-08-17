import {argsToAttrs} from "@utils/attributes";
import "@elements/core/share-jig/main";

export default {
    title: "Core / Share jig"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}


export const Main = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <share-jig-main ${argsToAttrs(props)}>
            <button-empty slot="close">&times;</button-empty>
            <share-jig-option kind="students"></share-jig-option>
            <share-jig-option kind="embed"></share-jig-option>
            <share-jig-option kind="copy"></share-jig-option>
        </share-jig-main>
    `;
}
Main.args = DEFAULT_ARGS;
