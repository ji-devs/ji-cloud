
export interface TreeNode {
    label: string,
    open: boolean,
    children: Array<TreeNode>,
    mode: "checkbox" | "inputText"}

export const mockHierarchy: Array<TreeNode> = [
    {
        label: "A",
        open: true,
        mode: "inputText",
        children: [
            {
                label: "A.1",
                open: false,
                mode: "inputText",
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
                    mode: "inputText"
                }]
            },
            {
                label: "A.2",
                open: true,
                mode: "inputText",
                children: [{
                    label: "A.2.1",
                    open: false,
                    children: [],
                    mode: "inputText"
                },
                {
                    label: "A.2.2",
                    open: false,
                    children: [],
                    mode: "inputText"
                }]
            }
        ]
    },
    {
        label: "B",
        open: true,
        mode: "inputText",
        children: [
            {
                label: "B.1",
                open: true,
                mode: "checkbox",
                children: [{
                    label: "B.1.1",
                    open: false,
                    children: [],
                    mode: "inputText"
                },
                {
                    label: "B.1.2",
                    open: false,
                    children: [],
                    mode: "inputText"
                }]
            },
            {
                label: "B.2",
                open: false,
                mode: "inputText",
                children: [{
                    label: "B.2.1",
                    open: false,
                    children: [],
                    mode: "inputText"
                },
                {
                    label: "B.2.2",
                    open: false,
                    children: [],
                    mode: "inputText"
                }]
            }
        ]
    }
]