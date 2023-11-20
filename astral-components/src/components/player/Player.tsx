import {PlaybackControls} from "./PlaybackControls.tsx";
import {usePlaylistController} from "../../util/PlaylistController.tsx";
import {TrackDisplay} from "./TrackDisplay.tsx";
import {usePalette} from "react-palette";
import {coverUrl} from "../../util/PlaylistController.tsx";
import React from "react";
import {AudioBar} from "./AudioBar.tsx";
import './AudioBar.css'
import {VolumeBar} from "./VolumeBar.tsx";
import {TimeDisplay} from "./TimeDisplay.tsx";

export const Player = () => {
    const { currentTrack } = usePlaylistController();
    const cover = coverUrl(currentTrack());
    const state = usePalette(cover)

    /// The player bar
    return (<div
        style={{
            boxShadow: `0 40px 150px 0 ${state.data.vibrant || "#aa7cf4"}63`,
            transition: `box-shadow 2s cubic-bezier(0.44, 0.31, 0.15, 0.94)`,
        } as React.CSSProperties}
        className={`w-[95%] ml-[3rem] mb-5 bg-zinc-950 rounded-lg h-[10%] bottom-0 absolute items-center flex flex-row`}
    >
        <AudioBar />
        <div className={"ml-5 mr-0 w-[20%]"}>
            <TrackDisplay track={currentTrack()} />
        </div>
        <div className={"m-auto w-fit mb-0 pb-0 h-[50%]"}>
            <PlaybackControls />
        </div>
        <div className={"mr-5 mx-auto w-[10%]"}>
            <VolumeBar />
            <TimeDisplay />
        </div>
    </div>)
}