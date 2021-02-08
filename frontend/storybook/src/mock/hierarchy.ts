
export interface TreeNode {
    label: string,
    open: boolean,
    children: Array<TreeNode>,
    mode: "checkbox" | "textInput" | "textDisplay"}

export const mockCategoryHierarchy: Array<TreeNode> = [
    {
        label: "A",
        open: true,
        mode: "textDisplay",
        children: [
            {
                label: "A.1",
                open: false,
                mode: "textDisplay",
                children: [{
                    label: "A.1.1",
                    open: false,
                    children: [],
                    mode: "textDisplay"
                },
                {
                    label: "A.1.2",
                    open: false,
                    children: [],
                    mode: "textDisplay"
                }]
            },
            {
                label: "A.2",
                open: true,
                mode: "textInput",
                children: [{
                    label: "A.2.1",
                    open: false,
                    children: [],
                    mode: "textDisplay"
                },
                {
                    label: "A.2.2",
                    open: false,
                    children: [],
                    mode: "textInput"
                }]
            }
        ]
    },
    {
        label: "B",
        open: true,
        mode: "textDisplay",
        children: [
            {
                label: "B.1",
                open: true,
                mode: "textDisplay",
                children: [{
                    label: "B.1.1",
                    open: false,
                    children: [],
                    mode: "textDisplay"
                },
                {
                    label: "B.1.2",
                    open: false,
                    children: [],
                    mode: "textDisplay"
                }]
            },
            {
                label: "B.2",
                open: false,
                mode: "textDisplay",
                children: [{
                    label: "B.2.1",
                    open: false,
                    children: [],
                    mode: "textInput"
                },
                {
                    label: "B.2.2",
                    open: false,
                    children: [],
                    mode: "textDisplay"
                }]
            }
        ]
    }
]

export const mockImagesHierarchy: Array<TreeNode> = [
    {
        label: "A",
        open: true,
        mode: "checkbox",
        children: [
            {
                label: "A.1",
                open: false,
                mode: "textDisplay",
                children: [{
                    label: "A.1.1",
                    open: false,
                    children: [],
                    mode: "checkbox"
                },
                {
                    label: "A.1.2",
                    open: false,
                    children: [],
                    mode: "checkbox"
                }]
            },
            {
                label: "A.2",
                open: true,
                mode: "textDisplay",
                children: [{
                    label: "A.2.1",
                    open: false,
                    children: [],
                    mode: "checkbox"
                },
                {
                    label: "A.2.2",
                    open: false,
                    children: [],
                    mode: "checkbox"
                }]
            }
        ]
    },
    {
        label: "B",
        open: true,
        mode: "textDisplay",
        children: [
            {
                label: "B.1",
                open: true,
                mode: "textDisplay",
                children: [{
                    label: "B.1.1",
                    open: false,
                    children: [],
                    mode: "checkbox"
                },
                {
                    label: "B.1.2",
                    open: false,
                    children: [],
                    mode: "checkbox"
                }]
            },
            {
                label: "B.2",
                open: false,
                mode: "textDisplay",
                children: [{
                    label: "B.2.1",
                    open: false,
                    children: [],
                    mode: "checkbox"
                },
                {
                    label: "B.2.2",
                    open: false,
                    children: [],
                    mode: "checkbox"
                }]
            }
        ]
    }
]