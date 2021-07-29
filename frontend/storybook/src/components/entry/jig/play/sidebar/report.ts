import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/report";

export default {
    title: "Entry / Jig / Play / Sidebar"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}


export const Report = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-report ${argsToAttrs(props)}>
            <select slot="select">
                <option>Offensive</option>
                <option>Copyright Infringement</option>
                <option>Spam</option>
            </select>
            <button slot="button">Report</button>
        </jig-play-sidebar-report>
    `;
}
Report.args = DEFAULT_ARGS;
