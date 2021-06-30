import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-audio/jig-audio-body";
import {Kind} from "@elements/entry/jig/edit/sidebar/settings/jig-audio/jig-audio-body";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Entry / Jig / Edit / Sidebar/ Settings/ Jig Audio"
}

interface Args {
    kind: Kind,
}

const DEFAULT_ARGS:Args = {
    kind: "feedback"
}

export const Body = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-audio-body ${argsToAttrs(props)}>
            <label slot="correct-mistake">
                <input type="radio">
                Correct answer
            </label>
            <label slot="correct-mistake">
                <input type="radio">
                Mistake
            </label>
            <button-rect kind="text" slot="back">Back to JIG settings</button-rect>
            <button-icon icon="x" slot="close"></button-icon>
            <button-rect kind="text" slot="add-custom">Add your own</button-rect>
            <input-search slot="search"></input-search>
            ${mapToString(arrayCount(7), line)}
        </jig-audio-body>
    `;
}
Body.args = DEFAULT_ARGS;
Body.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["background", "feedback"],
        }
    },
}



function line() {
    return `
        <jig-audio-line slot="lines" label="Some label">
            <jig-audio-play-pause mode="play" slot="play-pause"></jig-audio-play-pause>
        </jig-audio-line>
    `;
}
