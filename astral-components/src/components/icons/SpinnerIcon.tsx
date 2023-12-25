// icon:spinner-two | CSS Icons https://css.gg/ | Astrit
import * as React from "react";

export default function SpinnerIcon(props: React.SVGProps<SVGSVGElement>) {
    return (
        <svg fill="none" viewBox="0 0 24 24" height="2em" width="2em" {...props}>
            <path
                fillRule="evenodd"
                d="M12 19a7 7 0 100-14 7 7 0 000 14zm0 3c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"
                clipRule="evenodd"
                opacity={0.2}
            />
            <path
                d="M12 22c5.523 0 10-4.477 10-10h-3a7 7 0 01-7 7v3zM2 12C2 6.477 6.477 2 12 2v3a7 7 0 00-7 7H2z"
            />
        </svg>
    );
}