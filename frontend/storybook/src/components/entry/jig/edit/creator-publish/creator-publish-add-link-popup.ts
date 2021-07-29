import "@elements/entry/jig/edit/publish/add-link-popup";


import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Entry / Jig / Edit / Publish"
}

interface Args {
    open: boolean;
}

const DEFAULT_ARGS:Args = {
    open: true,
}

export const PublishAddLinkPopup = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-edit-publish-add-link-popup ${argsToAttrs(props)} style="margin:50px;">
            <button-icon slot="dismiss-action" icon="x"></button-icon>
            <textarea slot="textarea">Save</textarea>
            <button-rect kind="text" slot="action-cancel">Cancel</button-rect>
            <button-rect slot="action-save" color="blue">Save</button-rect>
        </jig-edit-publish-add-link-popup>
    `;
}

PublishAddLinkPopup.args = DEFAULT_ARGS;
