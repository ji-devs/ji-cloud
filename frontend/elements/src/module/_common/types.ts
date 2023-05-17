export type ModuleKind =
    | "cover"
    | "resource-cover"
    | "flashcards"
    | "matching"
    | "memory"
    | "poster"
    | "tapping-board"
    | "tracing"
    | "video"
    | "embed"
    | "card-quiz"
    | "drag-drop"
    | "find-answer";

export const moduleKinds: Array<ModuleKind> = [
    "cover",
    "resource-cover",
    "flashcards",
    "matching",
    "memory",
    "poster",
    "tapping-board",
    "tracing",
    "video",
    "embed",
    "card-quiz",
    "drag-drop",
    "find-answer",
];

//TODO - move these to JSON

export const STR_MODULE_DISPLAY_NAME: { [key in ModuleKind]: string } = {
    cover: "Cover",
    "resource-cover": "Cover",
    flashcards: "Flashcards",
    matching: "Matching",
    memory: "Memory Game",
    poster: "Talking Poster",
    "tapping-board": "Listen & Learn",
    tracing: "Tracing",
    video: "Video Player",
    embed: "Embed It",
    "card-quiz": "Multiple Choice",
    "drag-drop": "Drag & Drop",
    "find-answer": "Answer This",
};

export const STR_MODULE_CHOOSE_HEADER: { [key in ModuleKind]: string } = {
    cover: "", //not used
    "resource-cover": "",
    flashcards: "Create a set of Flashcards",
    matching: "Create a Matching Game",
    memory: "Create a Memory Game",
    "card-quiz": "Create a Multiple Choice activity",
    "tapping-board": "Create a Listen & Learn activity",
    poster: "Create a Talking Poster",
    "drag-drop": "Create a Drag & Drop activity",
    tracing: "Create a Tracing activity",
    video: "Create a Video activity",
    embed: "Create an Embed activity",
    "find-answer": "Create an Answer This activity",
};

export type AssetType = "jig" | "course" | "pro-dev";

export const STR_ASSET_TYPE: { [key in AssetType]: string } = {
    jig: "jig",
    course: "course",
    "pro-dev": "pro-dev"
};

export type JigFocus = "modules" | "resources";
