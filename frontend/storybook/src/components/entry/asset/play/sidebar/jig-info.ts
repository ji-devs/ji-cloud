import { argsToAttrs } from "@utils/attributes";
// import "@elements/entry/asset/play/jig/sidebar/jig-info";

export default {
    title: "Entry / Jig / Play / Sidebar",
};

interface Args {
    name: string;
    playedCount: number;
    likedCount: number;
    ages: string;
    language: string;
    byJiTeam: boolean;
    author: string;
    description: string;
}

const DEFAULT_ARGS: Args = {
    name: "The Big Gematria challenge",
    playedCount: 10,
    likedCount: 20,
    ages: "5-8",
    language: "english",
    byJiTeam: false,
    author: "Corinne",
    description:
        "This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry",
};

export const JigInfo = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-jig-info ${argsToAttrs(props)}>
            <button-empty slot="close">&times;</button-empty>
            <pill-close slot="categories" label="Hebrew"></pill-close>
            <pill-close slot="categories" label="Letters"></pill-close>
            <pill-close slot="categories" label="Letter recognition"></pill-close>
            <pill-close slot="categories" label="Holidays"></pill-close>
            <pill-close slot="categories" label="Passover"></pill-close>
            <pill-close slot="categories" label="Jewish Texts"></pill-close>
            <button-rect kind="text" slot="playlists">Sefer Bereishit</button-rect>
            <button-rect kind="text" slot="playlists">Shabat</button-rect>
            <button-rect kind="text" slot="playlists">Briat Haolam</button-rect>
            <button-rect slot="report" color="blue">Report</button-rect>
        </jig-play-sidebar-jig-info>
    `;
};
JigInfo.args = DEFAULT_ARGS;
