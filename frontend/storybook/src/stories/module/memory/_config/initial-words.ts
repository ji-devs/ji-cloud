import INITIAL_WORDS_JSON from "@frontend-config/module/memory/initial-words.json";

export const getInitialWords = ():string[] => {
    return (INITIAL_WORDS_JSON as any).words;
}