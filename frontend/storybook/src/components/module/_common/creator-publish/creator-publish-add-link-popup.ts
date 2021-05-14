import "@elements/module/_common/creator-publish/creator-publish-add-link-popup";


import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Creator Publish"
}

interface Args {
    open: boolean;
}

const DEFAULT_ARGS:Args = {
    open: true,
}

export const CreatorPublishAddLinkPopup = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <creator-publish-add-link-popup ${argsToAttrs(props)} style="margin:50px;">
            <button-icon slot="dismiss-action" icon="x"></button-icon>
            <textarea slot="textarea">Save</textarea>
            <button-text slot="action-cancel">Cancel</button-text>
            <button-rect slot="action-save" color="blue">Save</button-rect>
        </creator-publish-add-link-popup>
    `;
}

CreatorPublishAddLinkPopup.args = DEFAULT_ARGS;
