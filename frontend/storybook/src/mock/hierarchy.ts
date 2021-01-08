
export interface TreeNode {
    label: string,
    open: boolean,
    children: Array<TreeNode>
}

export const mockHierarchy: Array<TreeNode> = [
    {
        label: "A",
        open: true,
        children: [
            {
                label: "A.1",
                open: false,
                children: [{
                    label: "A.1.1",
                    open: false,
                    children: []
                },
                {
                    label: "A.1.2",
                    open: false,
                    children: []
                }]
            },
            {
                label: "A.2",
                open: true,
                children: [{
                    label: "A.2.1",
                    open: false,
                    children: []
                },
                {
                    label: "A.2.2",
                    open: false,
                    children: []
                }]
            }
        ]
    },
    {
        label: "B",
        open: true,
        children: [
            {
                label: "B.1",
                open: true,
                children: [{
                    label: "B.1.1",
                    open: false,
                    children: []
                },
                {
                    label: "B.1.2",
                    open: false,
                    children: []
                }]
            },
            {
                label: "B.2",
                open: false,
                children: [{
                    label: "B.2.1",
                    open: false,
                    children: []
                },
                {
                    label: "B.2.2",
                    open: false,
                    children: []
                }]
            }
        ]
    }
]