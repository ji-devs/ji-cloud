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
    | "card-quiz"
    | "drag-drop";

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
    "card-quiz",
    "drag-drop",
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
    "card-quiz": "Quiz Game",
    "drag-drop": "Drag & Drop",
};

export const STR_MODULE_CHOOSE_HEADER: { [key in ModuleKind]: string } = {
    cover: "", //not used
    "resource-cover": "",
    flashcards: "Create a Set of Flashcards",
    matching: "Create a Matching Game",
    memory: "Create a Memory Game",
    "card-quiz": "Create a Quiz Game",
    "tapping-board": "Create a Listen & Learn Activity",
    poster: "Create a Talking Poster",
    "drag-drop": "Create a Drag & Drop Activity",
    tracing: "Create a Tracing Activity",
    video: "Create a Video Activity",
};
