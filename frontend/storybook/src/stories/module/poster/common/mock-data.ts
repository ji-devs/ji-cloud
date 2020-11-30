interface Theme {
    id: string,
    label: string,
    thumbnail: string
}

export const mockImageThumbnail:string = `sticker-4991-2018-08-17-full.png`;

export const mockThemes:Theme[] = [
    {
        id: "bg-color",
        label: "Background Color",
        thumbnail: "Layout_Background_Color@2x.png",
    },
    {
        id: "bg-image",
        label: "Background Image",
        thumbnail: "Layout_Background_Image@2x.png",
    },
    {
        id: "2-horizontal-sections",
        label: "2 Horizontal Sections",
        thumbnail: "Layout_2_Horizontal_Sections@2x.png",
    },
    {
        id: "2-horizontal-sections-image",
        label: "2 Horizontal Sections",
        thumbnail: "Layout_2_Horizontal_Sections_Image@2x.png",
    },
    {
        id: "2-vertical-sections-eng",
        label: "2 Vertical Sections",
        thumbnail: "Layout_2_Vertical_Sections_Eng@2x.png",
    },
    {
        id: "2-vertical-sections-heb",
        label: "2 Vertical Sections",
        thumbnail: "Layout_2_Vertical_Sections_Heb@2x.png",
    },
    {
        id: "blank",
        label: "Blank",
        thumbnail: "Blank@2x.png",
    },
];