// icon:repeat-2-fill | Remix Icon https://remixicon.com/ | Remix Design
import * as React from "react";

export function RepeatCollectionButton(props: React.SVGProps<SVGSVGElement>) {
    return (
        <svg
            viewBox="0 0 24 24"
            fill="currentColor"
            height="1em"
            width="1em"
            {...props}
        >
            <path fill="none" d="M0 0h24v24H0z" />
            <path d="M8 20v1.932a.5.5 0 01-.82.385l-4.12-3.433A.5.5 0 013.382 18H18a2 2 0 002-2V8h2v8a4 4 0 01-4 4H8zm8-16V2.068a.5.5 0 01.82-.385l4.12 3.433a.5.5 0 01-.321.884H6a2 2 0 00-2 2v8H2V8a4 4 0 014-4h10z" />
        </svg>
    );
}

// icon:repeat-one-fill | Remix Icon https://remixicon.com/ | Remix Design
export function RepeatSingleButton(props: React.SVGProps<SVGSVGElement>) {
    return (
        <svg
            viewBox="0 0 24 24"
            fill="currentColor"
            height="1em"
            width="1em"
            {...props}
        >
            <path fill="none" d="M0 0h24v24H0z" />
            <path d="M8 20v1.932a.5.5 0 01-.82.385l-4.12-3.433A.5.5 0 013.382 18H18a2 2 0 002-2V8h2v8a4 4 0 01-4 4H8zm8-16V2.068a.5.5 0 01.82-.385l4.12 3.433a.5.5 0 01-.321.884H6a2 2 0 00-2 2v8H2V8a4 4 0 014-4h10zm-5 4h2v8h-2v-6H9V9l2-1z" />
        </svg>
    );
}

export interface RepeatButtonProps {
    repeat: 'collection' | 'single'
}

export default function RepeatButton(props: React.SVGProps<SVGSVGElement> & RepeatButtonProps) {
    return (
        props.repeat === 'collection' ? <RepeatCollectionButton {...props} /> : <RepeatSingleButton {...props} />
    )
}