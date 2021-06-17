import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/jig-info";

export default {
    title: "Entry / Jig / Play / Sidebar"
}

interface Args {
    name: string,
    playedCount: number,
    likedCount: number,
    ages: string,
    language: string,
    byJiTeam: boolean,
    author: string,
    description: string,
}

const DEFAULT_ARGS:Args = {
    name: "The Big Gematria challenge",
    playedCount: 10,
    likedCount: 20,
    ages: "5-8",
    language: "english",
    byJiTeam: false,
    author: "Corinne",
    description: "This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry",
}

export const JigInfo = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-jig-info ${argsToAttrs(props)}>
            <pill-close slot="categories" label="Hebrew"></pill-close>
            <pill-close slot="categories" label="Letters"></pill-close>
            <pill-close slot="categories" label="Letter recognition"></pill-close>
            <pill-close slot="categories" label="Holidays"></pill-close>
            <pill-close slot="categories" label="Passover"></pill-close>
            <pill-close slot="categories" label="Jewish Texts"></pill-close>
            <button-text slot="courses">Sefer Bereishit</button-text>
            <button-text slot="courses">Shabat</button-text>
            <button-text slot="courses">Briat Haolam</button-text>
            <button-rect slot="report" color="blue">Report</button-rect>
        </jig-play-sidebar-jig-info>
    `;
}
JigInfo.args = DEFAULT_ARGS;
