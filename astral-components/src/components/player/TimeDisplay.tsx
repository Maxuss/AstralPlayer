import {usePlaylistController} from "../../util/PlaylistController.tsx";
import {useEffect, useState} from "react";
import {useAbsoluteAudioTime} from "./AudioBar.tsx";

function formatTime(secs: number): string {
    return `${Math.floor(secs / 60).toString().padStart(2, "0")}:${Math.round(secs % 60).toString().padStart(2, "0")}`
}

export const TimeDisplay = () => {
    const { duration } = usePlaylistController();
    const progress = useAbsoluteAudioTime();
    const [time, setTime] = useState(["0:00", "0:00"])

    useEffect(() => {
        setTime([formatTime(progress), formatTime(duration)])
    }, [progress, duration])

    return <div className={"text-zinc-200 flex flex-row gap-5 ml-10"}>
        <p className={"w-5 mr-5"}>{time[0]}</p>
        <p>|</p>
        <p className={"w-5"}>{time[1]}</p>
    </div>
}