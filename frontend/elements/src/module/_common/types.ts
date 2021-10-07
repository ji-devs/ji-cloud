export type ModuleKind =
    "cover"
    | "flashcards"
    | "matching"
    | "memory"
    | "poster"
    | "tapping-board"
    | "tracing"
    | "video"
    | "card-quiz"
    | "drag-drop";

export const moduleKinds:Array<ModuleKind> = [ 
    "cover",
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

export const STR_MODULE_DISPLAY_NAME:{[key in ModuleKind]:string} = {
        "cover": "Cover",
        "flashcards": "Flashcards",
        "matching": "Matching",
        "memory": "Memory Game",
        "poster": "Poster",
        "tapping-board": "Tapping Board",
        "tracing": "Tracing",
        "video": "Video Player",
        "card-quiz": "Find the Pair",
        "drag-drop": "Drag and Drop",
};

export const STR_MODULE_CHOOSE_HEADER:{[key in ModuleKind]:string} = {
    "cover": "", //not used
    "flashcards": "Create a Flashcards Game",
    "matching": "Create a Quiz Game",
    "memory": "Create a Memory Game",
    "card-quiz": "Create a Quiz Game",
    "tapping-board": "Create a Tapping Board",
    "poster": "Create a Poster",
    "drag-drop": "Create a Drag and Drop",
    "tracing": "Create a Tracing",
    "video": "Create a Video",
};

export const STR_MODULE_PREVIEW_TOOLTIP_BODY:{[key in ModuleKind]:string} = {
    "cover": "Here’s your cover page for you to play. Want to change something? Just go back and edit!",
    "flashcards": "Here’s your flashcards for you to play. Want to change something? Just go back and edit!",
    "matching": "Here’s your matching game for you to play. Want to change something? Just go back and edit!",
    "memory": "Here’s your memory game for you to play. Want to change something? Just go back and edit!",
    "card-quiz": "Here’s your quiz for you to play. Want to change something? Just go back and edit!",
    "tapping-board": "Here’s your Tapping Board for you to play. Want to change something? Just go back and edit!",
    "poster": "Here’s your Poster for you to play. Want to change something? Just go back and edit!",
    "tracing": "Here’s your Tracing for you to play. Want to change something? Just go back and edit!",
    "drag-drop": "Here’s your Drag & Drop for you to play. Want to change something? Just go back and edit!",
    "video": "Here’s your Video for you to play. Want to change something? Just go back and edit!",
};