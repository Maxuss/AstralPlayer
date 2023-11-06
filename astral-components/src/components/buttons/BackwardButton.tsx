// icon:backward-step | Fontawesome https://fontawesome.com/ | Fontawesome
import * as React from "react";

export default function BackwardButton(props: React.SVGProps<SVGSVGElement>) {
    return (
        <svg
            viewBox="0 0 320 512"
            fill="currentColor"
            height="1em"
            width="1em"
            {...props}
        >
            <path d="M267.5 440.6c9.5 7.9 22.8 9.7 34.1 4.4s18.4-16.6 18.4-29V96c0-12.4-7.2-23.7-18.4-29s-24.5-3.6-34.1 4.4l-192 160L64 241V96c0-17.7-14.3-32-32-32S0 78.3 0 96v320c0 17.7 14.3 32 32 32s32-14.3 32-32V271l11.5 9.6 192 160z" />
        </svg>
    );
}
