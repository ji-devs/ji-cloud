
export const mockWords = ["שמש", "world", "שְׁמָע֕וּנִי", "blah blah blah"];

export const mockThemes = [
    {
        content: "שמש",
        id: "basic",
        label: "Basic",
    },
    {
        content: "שמש",
        id: "foo",
        label: "Foo",
    },
    {
        content: "שמש",
        id: "bar",
        label: "Bar",
    },
    {
      content: 'Word',
      id: 'orange',
      label: 'Orange',
    },
    {
      content: 'Dad',
      id: 'chalk',
      label: 'Chalk',
    }
];

export const nCardsToGrid = (nCards) => {
  switch(nCards) {
    case 8: return 1;
    case 10: return 2;
    case 12: return 1;
    case 14: return 5;
    case 16: return 1;
    case 18: return 6;
    case 20: return 2;
    case 22: return 7;
    case 24: return 3;
    case 26: return 8;
    case 28: return 4;
    default: return -1;
  }
};
