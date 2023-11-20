import axios from "axios";
import {useEffect, useState} from "react";
import {coverUrl, usePlaylistController} from "../../util/PlaylistController.tsx";
import './LyricsSidebar.css'
import {usePalette} from "react-palette";
import {useAbsoluteAudioTime} from "../player/AudioBar.tsx";

export interface SyncedLyricLine {
    start_time_ms: number,
    line: string
}

export type LyricsData = SyncedLyricLine[] | string[] | null;

async function requestLyrics(id: string): Promise<LyricsData> {
    // TODO: this is a placeholder

    const response = await axios({
        method: 'get',
        headers: {
            Authorization: `Bearer ${document.cookie.split("; ").find(row => row.startsWith("auth-token="))?.split("=")[1]}`,
        },
        url: `http://localhost:8080/lyrics/${id}`,
    }).catch(async (reject) => {
        if(reject.response.status === 401) {
            await refreshToken("v4.local.mb9WQj7Xn0RXCLEe3zCy0a-UZoUTTmxOitjWGshLGx8-zOaPRULD1DI1Ojo1B-Ot9AN3zNuZyJXVVCxu8CTIA0pHuiz2XaTY0hbzTDgcz1r-Qdu1dO-_9W5vSjKNP1fglFQtazkOcDKcWXBNZKhHSWD44i7WnTSG7sBu_pu3ci1cmNV4pOyLHpMWDbflheGLfWtN5GlpBbm38s9-cd3UnRfgZK0H6V108kF5h2nNIa3dLHyeitNa33MlbuOAyATtg-IoMy5LlRxQSBTjSyVl-1PlEjE_vdmg9ipuLB0rsFECa2o")
            return await requestLyrics(id)
        } else {
            return null
        }
    })
    return "data" in response! ? response.data.lines as LyricsData : response as LyricsData
}

async function refreshToken(access: string) {
    await axios({
        method: 'get',
        headers: {
            Authorization: `Bearer ${access}`,
        },
        url: 'http://localhost:8080/auth/token'
    })
}

export function useLyrics(): LyricsData {
    const { currentTrack } = usePlaylistController();
    const [lyrics, setLyrics] = useState<LyricsData>(null);

    useEffect(() => {
        requestLyrics(currentTrack()?.id || "00000000-0000-0000-0000-000000000000").then(setLyrics)
    }, [currentTrack]);

    return lyrics;
}

export const LyricsSidebar = () => {
    const { currentTrack, goto } = usePlaylistController();
    const lyrics = useLyrics();
    const { data } = usePalette(coverUrl(currentTrack()))
    const position = useAbsoluteAudioTime()
    const [currentIdx, setCurrentIdx] = useState(-1)
    const [latestScrolled, setLatestScrolled] = useState(-1)

    const findNearestSmallerNumber = (num: number, arr: number[]): [number, number] => {
        let smallest = -Infinity;
        let idx = -1;
        for (let i = 0; i < arr.length; i++) {
            if (arr[i] <= num && arr[i] >= smallest) {
                smallest = arr[i];
                idx = i;
            }
        }
        return [smallest, idx];
    };

    useEffect(() => {
        if(lyrics && typeof lyrics[0] === "object") {
            const el = document.querySelector('[data-id="lyrics-container"]')
            if(el) {
                const [nearestSmaller, idx] = findNearestSmallerNumber(position * 1000, lyrics.map(a => (a as SyncedLyricLine).start_time_ms));
                setCurrentIdx(idx)
                const sel = el
                    .querySelector(`[ms-pos="${nearestSmaller}"]`) as HTMLElement

                if (sel) {
                    const containerHeight = el.clientHeight;
                    const childTop = sel.offsetTop;
                    const childHeight = sel.offsetHeight;

                    const scrollTop = Math.max(
                        0,
                        childTop - (containerHeight - childHeight) / 4 - 15
                    );
                    if(latestScrolled === scrollTop)
                        return
                    setLatestScrolled(scrollTop)

                    el.scrollTo({
                        top: scrollTop - 20,
                        behavior: "smooth",
                    });

                }
            }
        }
    }, [position]);

    return <div className={`lyrics-frame w-[25.5%] h-[49rem] mt-[1rem] left-[72%]`} style={{ backgroundColor: data?.darkVibrant || "black" }}>
        <div className={"frame-lyrics"} data-id="lyrics-container">
            {
                lyrics?.map((each, idx) =>
                    typeof each === "object" ? (
                        <div
                            key={idx}
                            ms-pos={each.start_time_ms}
                            className={`frame-lyric-line transition-all linear duration-250 motion-reduce:transition-none ${each.line === "♪" ? "mx-[45%]" : ""} ${idx === currentIdx ? "frame-lyric-line-active" : ""} cursor-pointer`}
                            onClick={() => goto(each.start_time_ms / 1000)}
                            onMouseEnter={e => {
                                e.currentTarget.style.opacity = each.start_time_ms <= position * 1000 ? "90%" : "70%"
                            }}
                            onMouseLeave={e => {
                                e.currentTarget.style.opacity = "100%"
                            }}
                        >
                            {each.line}
                        </div>
                    ) : <div key={idx} className={"frame-lyric-line frame-lyric-line-active"}>{each}</div>
                ) ?? (<div className={"mt-[50%]"} >No lyrics for this song!</div>)
            }
        </div>

        <div className={"frame-bg"}>
            <img className="bg-color" alt={""} src={coverUrl(currentTrack()) || ""}/>
            <img className="bg-black" alt={""} src={coverUrl(currentTrack()) || ""}/>
        </div>
    </div>
}