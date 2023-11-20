import {coverUrl, QueuedTrack} from "../../util/PlaylistController.tsx";
import React from "react";

export interface DisplayProps {
    track: QueuedTrack | undefined
}

export const TrackDisplay: React.FC<DisplayProps> = ({ track }) => {
    if(track === undefined) {
        return (
            <div className={"animate-pulse flex items-center select-none"}>
                <div className={"w-[4rem] h-[4rem] rounded-lg bg-violet-300"}></div>
                <div className={"flex flex-col"}>
                    <div className={"w-[8rem] h-[0.6em] rounded-xl bg-violet-200 ml-2"}></div>
                    <div className={"w-[6rem] h-[0.4em] rounded-xl bg-violet-100 mt-[0.5rem] ml-2"}></div>
                </div>
            </div>
        )
    } else {
        return (
            <div className={"flex items-center select-none"}>
                <img className={"shadow-zinc-900 shadow-sm w-[4rem] h-[4rem] rounded-md"} src={coverUrl(track)} alt={`Cover art for ${track.album} by ${track.artist}`} />
                <div className={"flex flex-col"}>
                    <p className={"h-2 rounded-xl text-violet-100 ml-2 text-md cursor-pointer select-none"}>
                        {track.title}
                    </p>
                    <p className={"h-1 mt-[1rem] mb-[1rem] ml-2 text-sm text-violet-50 cursor-pointer select-none"}>
                        {track.artist}
                    </p>
                </div>
            </div>
        )
    }
}