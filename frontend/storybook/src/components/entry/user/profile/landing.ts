import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/profile/landing";
import "@elements/core/inputs/old/select";

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
            <input-wrapper slot="email">
                <input>
            </input-wrapper>
            <button-rect kind="outline" color="blue" size="small" slot="password-edit">Edit</button-rect>
            <input-wrapper slot="first-name">
                <input>
            </input-wrapper>
            <input-wrapper slot="family-name">
                <input>
            </input-wrapper>
            <input-wrapper slot="username">
                <input>
            </input-wrapper>
            <input-wrapper slot="location">
                <input>
            </input-wrapper>
            <input-select slot="preferred-language"></input-select>
            <input-wrapper slot="school-organization">
                <input>
            </input-wrapper>

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
