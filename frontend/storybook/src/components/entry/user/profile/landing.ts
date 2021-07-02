import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/profile/landing";
import "@elements/core/inputs/text-pencil";
import "@elements/core/inputs/dropdowns/dropdown-select";

export default {
    title: 'Entry / User / Profile',
}

interface Args {
    name: string,
    email: string,
}

const DEFAULT_ARGS:Args = {
    name: "Corinne Ossendryver",
    email: "corinne@jewishinteractive.net",
}

export const Profile = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <user-profile ${argsToAttrs(props)}>
            <img-ji slot="profile-image" lib="mock" id="face-round.webp" size="original"></img-ji>
            <img-ji slot="editable-profile-image" lib="mock" id="face-round.webp" size="original"></img-ji>
            <button-empty slot="profile-image-edit">✎</button-empty>
            <button-text slot="profile-image-delete">remove image</button-text>
            <input-text-pencil slot="email"></input-text-pencil>
            <button-rect kind="outline" color="blue" size="small" slot="password-edit">Edit</button-rect>
            <input-text-pencil slot="first-name"></input-text-pencil>
            <input-text-pencil slot="family-name"></input-text-pencil>
            <input-text-pencil slot="username"></input-text-pencil>
            <input-text-pencil slot="location"></input-text-pencil>
            <dropdown-select slot="preferred-language"></dropdown-select>
            <input-text-pencil slot="school-organization"></input-text-pencil>

            <pill-close slot="age-groups" label="Kindergarden"></pill-close>
            <pill-close slot="age-groups" label="Elementry School"></pill-close>
            <button-rect kind="outline" color="blue" size="small" slot="age-groups-edit">Edit</button-rect>

            <pill-close slot="relevant-subjects" label="Hebrew Language"></pill-close>
            <pill-close slot="relevant-subjects" label="Jewish Holidays"></pill-close>
            <button-rect kind="outline" color="blue" size="small" slot="relevant-subjects-edit">Edit</button-rect>

            <pill-close slot="affiliations" label="Reform content"></pill-close>
            <pill-close slot="affiliations" label="Conservative content"></pill-close>
            <pill-close slot="affiliations" label="Orthodox content"></pill-close>
            <pill-close slot="affiliations" label="Charedi content"></pill-close>
            <button-rect kind="outline" color="blue" size="small" slot="affiliations-edit">Edit</button-rect>
        </user-profile>
    `
}

Profile.args = DEFAULT_ARGS;
