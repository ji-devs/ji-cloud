import "@elements/entry/home/home/quick-search/quick-search-item";

export default {
    title: "Entry / Home / Home / Quick search",
};

export const QuickSearchItem = () => {
    return `
        <home-quick-search-item>
            <img-ji></img-ji>
            <h4 slot="title">Chanukah</h4>
            <h5 slot="subtitle">355 JIGs</h5>
        </home-quick-search-item>
    `;
};
