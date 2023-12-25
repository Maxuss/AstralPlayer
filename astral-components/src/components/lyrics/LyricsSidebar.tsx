import {useEffect, useState} from "react";
import {coverUrl, usePlaylistController} from "../../util/PlaylistController.tsx";
import './LyricsSidebar.css'
import {usePalette} from "react-palette";
import {useAbsoluteAudioTime} from "../player/AudioBar.tsx";
import {useBackendController} from "../../util/BackendController.tsx";

export interface SyncedLyricLine {
    start_time_ms: number,
    line: string
}

export type LyricsData = SyncedLyricLine[] | string[] | null;

export function useLyrics(): LyricsData {
    const { currentTrack } = usePlaylistController();
    const { get } = useBackendController();
    const [lastFetchedTrack, setLastFetchedTrack] = useState("00000000-0000-0000-0000-000000000000")
    const [lyrics, setLyrics] = useState<LyricsData>(null);

    useEffect(() => {
        const id = currentTrack()?.id || "00000000-0000-0000-0000-000000000000";
        if(lastFetchedTrack !== id)
            get(`/lyrics/${id}`).then(response => {
                const lyrics = response.lines as LyricsData
                setLyrics(lyrics);
                setLastFetchedTrack(id)
            }).catch(() => {
                setLyrics(null)
                setLastFetchedTrack(id)
            })
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

    return <div className={`lyrics-frame w-[25%] h-[87%] top-[1%] left-[72.5%]`} style={{ backgroundColor: data?.darkVibrant || "black" }}>
        <div className={"frame-lyrics"} data-id="lyrics-container">
            {
                lyrics?.map((each, idx) =>
                    typeof each === "object" ? (
                        <div
                            key={idx}
                            ms-pos={each.start_time_ms}
                            className={`frame-lyric-line 
                            transition-all linear duration-250 motion-reduce:transition-none ${each.line === "♪" ? "mx-[45%]" : ""}
                            ${idx === currentIdx ? "frame-lyric-line-active" : ""} cursor-pointer text-left`}
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
                )
            }
        </div>

        <div className={"frame-bg"}>
            <img className="bg-color" alt={""} src={coverUrl(currentTrack()) || ""}/>
            <img className="bg-black" alt={""} src={coverUrl(currentTrack()) || ""}/>
        </div>
    </div>
}