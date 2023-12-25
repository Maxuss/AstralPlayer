import {IndexedTrack} from "./SearchView.tsx";
import React, {useCallback, useState} from "react";
import {usePlaylistController} from "../../../util/PlaylistController.tsx";
import {useBackendController} from "../../../util/BackendController.tsx";
import {IconPlayFill} from "../../icons/IconPlayFill.tsx";
import {Love} from "../../icons/LoveIcon.tsx";

export interface TrackDisplayProps {
    index: IndexedTrack,
    // setView: (view: ViewType) => void
}

export const SearchTrackDisplay: React.FC<TrackDisplayProps> = ({ index }) => {
    const { append, next, currentTrack } = usePlaylistController();
    const { post } = useBackendController();
    const [isHovered, setHovered] = useState(false);
    const [isLoved, setLoved] = useState(index.loved);

    const playTrack = useCallback(() => {
        // TODO: fix this oh my god x2
        append({
            album: index.album_id, artist: index.artists.map(([each]) => each).join(", "), format: index.format as ("mp3" | "flac"), id: index.id, title: index.name
        })
        next()
    }, [append, index, next])

    return <div
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
        onClick={playTrack}
        className={"bg-[#3c3c3d40] playing-icon-outer cursor-pointer h-full w-[40rem] rounded-lg py-3 hover:bg-[#eeeeeeab] transition-colors ease-in-out flex flex-row text-zinc-200 hover:text-black track-hover-outer"}
    >
        <div className={"mx-5 mt-2 w-[10%] h-[10%]"}>
            {currentTrack()?.id === index.id ? (<div className={"playing-icon mt-1.5"}>
                <span className={"playing-bar transition-colors ease-in-out"} />
                <span className={"playing-bar transition-colors ease-in-out"} />
                <span className={"playing-bar transition-colors ease-in-out"} />
            </div>) : isHovered ? <IconPlayFill className={"mt-1 fill-black scale-[200%]"} /> : <img src={`http://localhost:8080/metadata/track/${index.id}/cover`} alt={" "} className={"mt-[-0.1em] rounded-lg scale-[150%]"} />}
        </div>
        <div className={"my-2 w-[150%]"}>{index.name}</div>
        <div className={"my-2 w-[40%]"}>{Math.floor(index.duration / 60)}:{(index.duration % 60).toString().padStart(2, "0")}</div>

        <Love love={isLoved} className={"fill-black-hover mt-3 transition-colors ease-in-out scale-[300%] cursor-pointer ml-5 mr-10"} onClick={async e => {
            e.stopPropagation()
            await post(`/user/${isLoved ? "unlove" : "love"}/track/${index.id}`, {}, 'POST').then(() => {
                setLoved(!isLoved)
            })
        }} />

    </div>

}