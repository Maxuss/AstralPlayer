// icon:circle-fill | Bootstrap https://icons.getbootstrap.com/ | Bootstrap
import * as React from "react";

function CircleFill(props: React.SVGProps<SVGSVGElement>) {
    return (
        <svg
            fill="currentColor"
            viewBox="0 0 16 16"
            height="1em"
            width="1em"
            {...props}
        >
            <path d="M16 8 A8 8 0 0 1 8 16 A8 8 0 0 1 0 8 A8 8 0 0 1 16 8 z" />
        </svg>
    );
}

// icon:circle-half | Bootstrap https://icons.getbootstrap.com/ | Bootstrap
function CircleHalf(props: React.SVGProps<SVGSVGElement>) {
    return (
        <svg
            fill="currentColor"
            viewBox="0 0 16 16"
            height="1em"
            width="1em"
            {...props}
        >
            <path d="M8 15A7 7 0 108 1v14zm0 1A8 8 0 118 0a8 8 0 010 16z" />
        </svg>
    );
}

// icon:circle | Bootstrap https://icons.getbootstrap.com/ | Bootstrap
function CircleEmpty(props: React.SVGProps<SVGSVGElement>) {
    return (
        <svg
            fill="currentColor"
            viewBox="0 0 16 16"
            height="1em"
            width="1em"
            {...props}
        >
            <path d="M8 15A7 7 0 118 1a7 7 0 010 14zm0 1A8 8 0 108 0a8 8 0 000 16z" />
        </svg>
    );
}

export interface CircleProps {
    style: 'full' | 'half' | 'empty',
}

export const CircleIcon: React.FC<CircleProps & React.SVGProps<SVGSVGElement>> = ({ style, ... props }) => {
    switch(style) {
        case 'full': return <CircleFill {...props} />
        case 'half': return <CircleHalf {...props} />
        case 'empty': return <CircleEmpty {...props} />
    }
}
