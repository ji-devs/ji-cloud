
export interface TreeNode {
    label: string,
    open: boolean,
    children: Array<TreeNode>,
    content: string
}

export const mockHierarchy: Array<TreeNode> = [
    {
        label: "A",
        open: true,
        content: "inputText",
        children: [
            {
                label: "A.1",
                open: false,
                content: "inputText",
                children: [{
                    label: "A.1.1",
                    open: false,
                    children: [],
                    content: "checkbox"
                },
                {
                    label: "A.1.2",
                    open: false,
                    children: [],
                    content: "inputText"
                }]
            },
            {
                label: "A.2",
                open: true,
                content: "inputText",
                children: [{
                    label: "A.2.1",
                    open: false,
                    children: [],
                    content: "inputText"
                },
                {
                    label: "A.2.2",
                    open: false,
                    children: [],
                    content: "inputText"
                }]
            }
        ]
    },
    {
        label: "B",
        open: true,
        content: "inputText",
        children: [
            {
                label: "B.1",
                open: true,
                content: "checkbox",
                children: [{
                    label: "B.1.1",
                    open: false,
                    children: [],
                    content: "inputText"
                },
                {
                    label: "B.1.2",
                    open: false,
                    children: [],
                    content: "inputText"
                }]
            },
            {
                label: "B.2",
                open: false,
                content: "inputText",
                children: [{
                    label: "B.2.1",
                    open: false,
                    children: [],
                    content: "inputText"
                },
                {
                    label: "B.2.2",
                    open: false,
                    children: [],
                    content: "inputText"
                }]
            }
        ]
    }
]