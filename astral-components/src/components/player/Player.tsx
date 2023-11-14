import {PlaybackControls} from "./PlaybackControls.tsx";
import {usePlaylistController} from "../../util/PlaylistController.tsx";
import {TrackDisplay} from "./TrackDisplay.tsx";
import {usePalette} from "react-palette";
import {useEffect} from "react";

export const Player = () => {
    const { currentTrack } = usePlaylistController();
    const coverUrl = currentTrack()?.coverUrl;
    const state = usePalette(coverUrl === undefined ? "" : coverUrl)

    useEffect(() => {
        console.log(state)
    }, [state]);

    /// The player bar
    return (<div
        style={{
            backgroundImage: `linear-gradient(to right, ${state.data.darkMuted}, transparent 20%)`
        }}
        className={`w-[100%] bg-zinc-950 rounded-t-lg h-[10%] bottom-0 absolute ml-0 items-center flex flex-row`}
    >
        <div className={"m-5 w-[15%]"}>
            <TrackDisplay track={currentTrack()} />
        </div>
        <div className={"m-auto w-[60%] mb-0"}>
            <PlaybackControls />
        </div>
    </div>)
}