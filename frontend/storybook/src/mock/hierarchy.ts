export interface TreeNode {
    label: string;
    expanded: boolean;
    children: Array<TreeNode>;
    mode: "checkbox" | "textInput" | "textDisplay";
    menuButton?: boolean;
    menuContents?: boolean;
    expandAllButton?: boolean;
}

export const mockTempHierarchy: Array<TreeNode> = [
    {
        label: "A",
        expanded: false,
        mode: "textDisplay",
        children: [],
    },
];

export const mockCategoryHierarchy: Array<TreeNode> = [
    {
        label: "A",
        expanded: true,
        mode: "textDisplay",
        menuButton: true,
        expandAllButton: true,
        children: [
            {
                label: "A.1",
                expanded: false,
                mode: "textDisplay",
                children: [
                    {
                        label: "A.1.1",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                    {
                        label: "A.1.2",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                ],
            },
            {
                label: "A.2",
                expanded: true,
                mode: "textInput",
                children: [
                    {
                        label: "A.2.1",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                    {
                        label: "A.2.2",
                        expanded: false,
                        children: [],
                        mode: "textInput",
                    },
                ],
            },
        ],
    },
    {
        label: "B",
        expanded: true,
        mode: "textDisplay",
        expandAllButton: true,
        children: [
            {
                label: "B.1",
                expanded: true,
                mode: "textDisplay",
                menuButton: true,
                menuContents: true,
                children: [
                    {
                        label: "B.1.1",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                    {
                        label: "B.1.2",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                ],
            },
            {
                label: "B.2",
                expanded: false,
                mode: "textDisplay",
                children: [
                    {
                        label: "B.2.1",
                        expanded: false,
                        children: [],
                        mode: "textInput",
                    },
                    {
                        label: "B.2.2",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                ],
            },
        ],
    },
    {
        label: "C",
        expanded: false,
        mode: "textDisplay",
        expandAllButton: true,
        children: [
            {
                label: "C.1",
                expanded: true,
                mode: "textDisplay",
                menuButton: true,
                menuContents: true,
                children: [
                    {
                        label: "C.1.1",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                    {
                        label: "C.1.2",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                ],
            },
            {
                label: "C.2",
                expanded: false,
                mode: "textDisplay",
                children: [
                    {
                        label: "B.2.1",
                        expanded: false,
                        children: [],
                        mode: "textInput",
                    },
                    {
                        label: "C.2.2",
                        expanded: false,
                        children: [],
                        mode: "textDisplay",
                    },
                ],
            },
        ],
    },
];

export const mockImagesHierarchy: Array<TreeNode> = [
    {
        label: "A",
        expanded: true,
        mode: "checkbox",
        expandAllButton: true,
        children: [
            {
                label: "A.1",
                expanded: false,
                mode: "textDisplay",
                children: [
                    {
                        label: "A.1.1",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                    {
                        label: "A.1.2",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                ],
            },
            {
                label: "A.2",
                expanded: true,
                mode: "textDisplay",
                children: [
                    {
                        label: "A.2.1",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                    {
                        label: "A.2.2",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                ],
            },
        ],
    },
    {
        label: "B",
        expanded: true,
        mode: "textDisplay",
        expandAllButton: true,
        children: [
            {
                label: "B.1",
                expanded: true,
                mode: "textDisplay",
                children: [
                    {
                        label: "B.1.1",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                    {
                        label: "B.1.2",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                ],
            },
            {
                label: "B.2",
                expanded: false,
                mode: "textDisplay",
                children: [
                    {
                        label: "B.2.1",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                    {
                        label: "B.2.2",
                        expanded: false,
                        children: [],
                        mode: "checkbox",
                    },
                ],
            },
        ],
    },
];
