import GRID_LOOKUP_JSON from "@frontend-config/module/memory/grid-lookup.json";

export interface GridLookup {
    [key: string]: string
}

export const nCardsToGrid = (nCards:number):string => {
    const lookup:GridLookup = GRID_LOOKUP_JSON.grid;
    return lookup[`${nCards}`];
}