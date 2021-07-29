export const letters = ["alef", "vet", "bet", "gimel", "dalet", "hay", "vav", "vav-holam", "vav-shuruk", "zayin", "chet", "tet", "yud", "chaf", "kaf", "chaf-sofit", "lamed", "mem", "mem-sofit", "nun", "nun-sofit", "samech", "ayin", "fay", "pay", "fay-sofit", "tsadi", "tsadi-sofit", "kuf", "resh", "shin-sin", "shin", "sin", "tav"] as const;
export type Letter = typeof letters[number];

export const niqquds = ["dagesh", "patach", "kamatz", "kamatz-katan", "‎sheva", "chirik", "tsere", "segol", "hataf-patach", "hataf-kamatz", "hataf-segol", "cholam-chaser", "kubutz"] as const;
export type Niqqud = typeof niqquds[number];

export const punctuations = ['dot', 'comma', 'geresh'] as const;
export type Punctuation = typeof punctuations[number];

export const cantillations = ['atnah', 'zakef-katan', 'oleh', 'meteg-and-siluk'] as const;
export type Cantillation = typeof cantillations[number];


export interface NiqqudInfo {
    name: string,
    char: string,
}

export const NIQQUD_INFO: {
    [key in Niqqud]: NiqqudInfo
} = {
    "dagesh": {
        name: "Dagesh",
        char: "ּ",
    },
    "patach": {
        name: "Patach",
        char: "ַ",
    },
    "kamatz": {
        name: "Kamatz",
        char: "ָ",
    },
    "kamatz-katan": {
        name: "Kamatz katan",
        char: "ׇ",
    },
    "‎sheva": {
        name: "‎sheva",
        char: "ְ",
    },
    "chirik": {
        name: "Chirik",
        char: "ִ",
    },
    "tsere": {
        name: "Tsere",
        char: "ֵ",
    },
    "segol": {
        name: "Segol",
        char: "ֶ",
    },
    "hataf-patach": {
        name: "Hataf patach",
        char: "ֲ",
    },
    "hataf-kamatz": {
        name: "Hataf kamatz",
        char: "ֳ",
    },
    "hataf-segol": {
        name: "Hataf segol",
        char: "ֱ",
    },
    "cholam-chaser": {
        name: "Cholam chaser", 
        char: "ֹ",
    },
    "kubutz": {
        name: "Kubutz",
        char: "ֻ",
    },
};


export interface LetterInfo {
    name: string,
    char: string,
}

export const LETTER_INFO: {
    [key in Letter]: LetterInfo
} = {
    "alef": {
        name: "Alef",
        char: 'א',
    },
    "vet": {
        name: "Vet",
        char: 'ב',
    },
    "bet": {
        name: "Bet",
        char: 'בּ',
    },
    "gimel": {
        name: "Gimel",
        char: 'ג',
    },
    "dalet": {
        name: "Dalet",
        char: 'ד',
    },
    "hay": {
        name: "Hay",
        char: 'ה',
    },
    "vav": {
        name: "Vav",
        char: 'ו',
    },
    "vav-holam": {
        name: "Cholom",
        char: 'וֹ',
    },
    "vav-shuruk": {
        name: "Shuruk",
        char: 'וּ',
    },
    "zayin": {
        name: "Zayin",
        char: 'ז',
    },
    "chet": {
        name: "Chet",
        char: 'ח',
    },
    "tet": {
        name: "Tet",
        char: 'ט',
    },
    "yud": {
        name: "Yud",
        char: 'י',
    },
    "chaf": {
        name: "Chaf",
        char: 'כ',
    },
    "kaf": {
        name: "Kaf",
        char: 'כּ',
    },
    "chaf-sofit": {
        name: "Khaf Sofit",
        char: 'ך',
    },
    "lamed": {
        name: "Lamed",
        char: 'ל',
    },
    "mem": {
        name: "Mem",
        char: 'מ',
    },
    "mem-sofit": {
        name: "Mem Sofit",
        char: 'ם',
    },
    "nun": {
        name: "Nun",
        char: 'נ',
    },
    "nun-sofit": {
        name: "Nun Sofit",
        char: 'ן',
    },
    "samech": {
        name: "Samech",
        char: 'ס',
    },
    "ayin": {
        name: "Ayin",
        char: 'ע',
    },
    "fay": {
        name: "Fay",
        char: 'פ',
    },
    "pay": {
        name: "Pay",
        char: 'פּ',
    },
    "fay-sofit": {
        name: "Fay Sofit",
        char: 'ף',
    },
    "tsadi": {
        name: "Tsadi",
        char: 'צ',
    },
    "tsadi-sofit": {
        name: "Tsadi Sofit",
        char: 'ץ',
    },
    "kuf": {
        name: "Kuf",
        char: 'ק',
    },
    "resh": {
        name: "Resh",
        char: 'ר',
    },
    "shin-sin": {
        name: "Shin",
        char: 'ש',
    },
    "shin": {
        name: "Shin",
        char: 'שׁ',
    },
    "sin": {
        name: "Sin",
        char: 'שׂ',
    },
    "tav": {
        name: "Tav",
        char: 'ת',
    },
};


export interface PunctuationInfo {
    name: string,
    char: string,
}

export const PUNCTUATION_INFO: {
    [key in Punctuation]: PunctuationInfo
} = {
    "dot": {
        name: "Dot",
        char: '.',
    },
    "comma": {
        name: "Comma",
        char: ',',
    },
    "geresh": {
        name: "Beth",
        char: '`',
    },
};


export interface CantillationInfo {
    name: string,
    char: string,
}

export const CANTILLATION_INFO: {
    [key in Cantillation]: CantillationInfo
} = {
    "atnah": {
        name: "Atnah",
        char: "֑"
    },
    "zakef-katan": {
        name: "Zakef katan",
        char: "֔"
    },
    "oleh": {
        name: "Oleh",
        char: "֫"
    },
    "meteg-and-siluk": {
        name: "Meteg & siluk",
        char: "ֽ"
    },
};
