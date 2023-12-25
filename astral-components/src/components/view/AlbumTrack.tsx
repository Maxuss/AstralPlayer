import {AlbumTrackData} from "./AlbumView.tsx";
import React, {useCallback, useState} from "react";
import {usePlaylistController} from "../../util/PlaylistController.tsx";
import {IconPlayFill} from "../icons/IconPlayFill.tsx";
import {Love} from "../icons/LoveIcon.tsx";
import './AlbumTrack.css';

export interface AlbumTrackProps {
    data: AlbumTrackData,
    album: string,
}

export const AlbumTrack: React.FC<AlbumTrackProps> = ({ data, album }) => {
    const { append, next, currentTrack } = usePlaylistController();
    const [isHovered, setHovered] = useState(false);

    const playTrack = useCallback(() => {
        // TODO: fix this oh my god
        append({
            album: album, artist: data.artist, format: data.format as ("mp3" | "flac"), id: data.id, title: data.name
        })
        next()
    }, [append, album, data.artist, data.format, data.id, data.name, next])

    return <div
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
        onClick={playTrack}
        className={"bg-[#3c3c3d40] playing-icon-outer cursor-pointer h-full w-[40rem] rounded-lg py-3 hover:bg-[#eeeeeeab] transition-colors ease-in-out flex flex-row text-zinc-200 hover:text-black track-hover-outer"}
    >
        <div className={"mx-5 mt-2 w-[10%] h-[10%]"}>
            {currentTrack()?.id === data.id ? (<div className={"playing-icon mt-1.5"}>
                <span className={"playing-bar transition-colors ease-in-out"} />
                <span className={"playing-bar transition-colors ease-in-out"} />
                <span className={"playing-bar transition-colors ease-in-out"} />
            </div>) : isHovered ? <IconPlayFill className={"mt-1 fill-black scale-[200%]"} /> : data.index}
        </div>
        <div className={"my-2 w-[150%]"}>{data.name}</div>
        <div className={"my-2 w-[40%]"}>{Math.floor(data.length / 60)}:{(data.length % 60).toString().padStart(2, "0")}</div>

        <Love love={false} className={"fill-black-hover mt-3 transition-colors ease-in-out scale-[300%] cursor-pointer ml-5 mr-10"} />

    </div>
}