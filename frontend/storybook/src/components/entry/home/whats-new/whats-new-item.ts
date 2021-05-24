import "@elements/entry/home/whats-new/home-new-item";



export default {
    title: "Entry/ Home / What's new",
}

const STR_SUBTITLE = "HOP TV - New Hebrew Series";
const STR_PARAGRAPH = "Learning Hebrew with HOP Channel, Learning Hebrew with HOP Channel, Learning Hebrew with HOP Channel, Learning Hebrew with HOP Channel Learning Hebrew with HOP ";
const STR_PLAY = "Play Series";

export const WhatsNewItem = () => {
    return `
        <home-new-item slot="items">
            <img-ji slot="image" id="something.jpg" lib="mock" size="original"></img-ji>
            <h2 slot="subtitle">${STR_SUBTITLE}</h2>
            <p slot="lines">${STR_PARAGRAPH}</p>
            <button-rect slot="button" size="large" color="red" bold>${STR_PLAY}</button-rect>
        </home-new-item>
    `
}
