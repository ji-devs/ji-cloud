import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/search-results/search-result";
import "@elements/entry/home/search-results/search-result-details";
import "@elements/entry/home/search-results/search-result-category";
import { Details } from "./search-result-details";

export default {
    title: "Entry / Home / Search results"
}

interface Args {
    new: boolean,
    leaningPathJigCount?: number,
    title: string,
    playedCount: number,
    likedCount: number,
    ages: string,
    language: string,
    byJiTeam: boolean,
    author: string,
    description: string,
}

const DEFAULT_ARGS:Args = {
    new: false,
    leaningPathJigCount: undefined,
    title: "The Big Gematria challenge",
    playedCount: 10,
    likedCount: 20,
    ages: "5-8",
    language: "english",
    byJiTeam: false,
    author: "Corinne",
    description: "This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry",
}

export const SearchResult = (props?:Args) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <home-search-result ${argsToAttrs(props)} slot="results">
            <img-ji lib="mock" size="full" id="jig-gallery.jpg" slot="image"></img-ji>
            <home-search-result-details slot="categories">
                <home-search-result-category label="Hebrew letters"></home-search-result-category>
                <home-search-result-category label="Hebrew reading"></home-search-result-category>
                <home-search-result-category label="Vocabulary"></home-search-result-category>
                <home-search-result-category label="Hebrew letters"></home-search-result-category>
            </home-search-result-details>
            <button-rect slot="play-button" color="bluewhite">Play</button-rect>
        </home-search-result>
    `;
}

SearchResult.args = DEFAULT_ARGS;
