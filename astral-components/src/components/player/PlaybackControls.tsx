import React, {useCallback, useEffect, useRef, useState} from "react";
import {PlayButton} from "../buttons/PlayButton.tsx";
import PauseButton from "../buttons/PauseButton.tsx";
import {useGlobalAudioPlayer} from "react-use-audio-player";
import Track1 from '../../../audio/Sakura.mp3';
import ShuffleButton from "../buttons/ShuffleButton.tsx";
import RepeatButton from "../buttons/RepeatButton.tsx";
import BackwardButton from "../buttons/BackwardButton.tsx";
import ForwardButton from "../buttons/ForwardButton.tsx";
import './PlaybackControls.css'
import {usePlaylistController} from "../../util/PlaylistController.tsx";

interface PlayPauseProps {
    isPlaying: boolean,
}

const PlayPause: React.FC<PlayPauseProps & React.SVGProps<SVGSVGElement>> = ({ isPlaying, ... props }) => {
    return isPlaying ? <PauseButton {...props} /> : <PlayButton {...props} />
}

export const PlaybackControls = () => {
    const { toggle, append, repeat, setRepeat, isShuffle, setShuffle, next, back, queue, isPlaying } = usePlaylistController();

    useEffect(() => {
        append({
            album: "Murmuüre", artist: "Murmuüre",
            coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/937c42239633c3106f3299edd7c20da6.jpg",
            format: "mp3",
            streamUrl: Track1,
            title: "Prima Vere"
        })
        append({
            album: "THE GHOST~POP TAPE", artist: "Devon Hendryx",
            coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/976bf708fe0c03cfd1c17adf8f670d28.jpg",
            format: "mp3",
            streamUrl: Track1,
            title: "Bubblegum Crisis"
        })
        append({
            album: "Down Below", artist: "Tribulation",
            coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/5390a5a5cbef2a585da49609fd511d70.jpg",
            format: "mp3",
            streamUrl: Track1,
            title: "The Lament"
        })
        append({
            album: "World Coming Down", artist: "Type O Negative",
            coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/c31e9911d92c44a7b6312ceb156bf78d.jpg",
            format: "mp3",
            streamUrl: Track1,
            title: "Pyretta Blaze"
        })
        append({
            album: "Konkurs", artist: "Lifelover",
            coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/d2e8cc6713ee2fd861936bc0fb81deab.jpg",
            format: "mp3",
            streamUrl: Track1,
            title: "Mental Central Dialog"
        })
        append({
            album: "Sworn to the Dark", artist: "Watain",
            coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/333ee44a25a205514d4b4ccfa9e57f2b.jpg",
            format: "mp3",
            streamUrl: Track1,
            title: "Satan's Hunger"
        })
        append({
            album: "fishmonger", artist: "underscores",
            coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/0074590f78c850626134e0c01b3af7d1.jpg",
            format: "mp3",
            streamUrl: Track1,
            title: "Second hand embarassment"
        })

        next()
    }, [queue])


    return (<div className="flex flex-col gap-5 w-[50rem]">
        <span className={"fill-white flex flex-row gap-10 self-center"}>
            <ShuffleButton
                className={`
                    ${isShuffle ? "fill-[#9573f4] hover:fill-violet-400" : "fill-zinc-400 hover:fill-violet-200"} 
                    transition-colors ease-in-out scale-[150%] cursor-pointer
                `}
                onClick={() => { setShuffle(!isShuffle) }}
            />
            <BackwardButton
                className="fill-zinc-400 hover:fill-violet-200 transition-colors ease-in-out scale-[150%] cursor-pointer"
                onClick={back}
            />
            <PlayPause
                isPlaying={isPlaying}
                className="fill-white hover:fill-violet-200 transition-colors ease-in-out scale-[250%] cursor-pointer"
                onClick={toggle}
            />
            <ForwardButton
                className="fill-zinc-400 hover:fill-violet-200 transition-colors ease-in-out scale-[150%] cursor-pointer"
                onClick={next}
            />
            <RepeatButton
                repeat={repeat === 'disabled' ? 'collection' : repeat}
                className={`
                    ${repeat !== 'disabled' ? "fill-[#9573f4] hover:fill-violet-400" : "fill-zinc-400 hover:fill-violet-200"} 
                    transition-colors ease-in-out scale-[160%] cursor-pointer
                `}
                onClick={() => {
                    if(repeat === 'disabled')
                        setRepeat('collection')
                    else if(repeat === 'collection')
                        setRepeat('single')
                    else
                        setRepeat('disabled')
                }}
            />
        </span>
        <AudioProgressBar moreTailwind={""} />
    </div>)
}

function useRelativeAudioTime() {
    const frameRef = useRef<number>()
    const [pos, setPos] = useState(0)
    const { getPosition, duration } = useGlobalAudioPlayer()

    useEffect(() => {
        const animate = () => {
            setPos(getPosition() / duration)
            frameRef.current = requestAnimationFrame(animate)
        }

        frameRef.current = window.requestAnimationFrame(animate)

        return () => {
            if (frameRef.current) {
                cancelAnimationFrame(frameRef.current)
            }
        }
    }, [getPosition, duration])

    return pos;
}

interface ProgressBarProps {
    moreTailwind: string
}

export const AudioProgressBar: React.FC<ProgressBarProps> = ({ moreTailwind }: ProgressBarProps) => {
    const { getPosition, goto, duration } = usePlaylistController();
    const [time, setTime] = useState(["0:00", "0:00"])
    const [vars, setVars] = useState({ "--progress-bar-transform": "0%" } as React.CSSProperties)
    const [progressUpdating, setProgressUpdating] = useState(false);
    const [intermediatePosition, setIntermediatePosition] = useState(0)
    const progress = useRelativeAudioTime()
    const parentElement = useRef<HTMLDivElement>(null)

    const handlePointerMove = useCallback((e: PointerEvent) => {
        if(progressUpdating)
            handleMove(e)
    }, [progressUpdating])

    const handlePointerUp = useCallback((e: PointerEvent) => {
        if(progressUpdating) {
            setProgressUpdating(false)
            handleMove(e)
            goto(intermediatePosition * duration)
        }
    }, [progressUpdating, goto, intermediatePosition, duration, setProgressUpdating])

    useEffect(() => {
        window.addEventListener("pointermove", handlePointerMove)
        window.addEventListener("pointerup", handlePointerUp)

        return () => {
            // i have spent over 4 hours debugging because of not handling unmounting
            window.removeEventListener("pointermove", handlePointerMove)
            window.removeEventListener("pointerup", handlePointerUp)
        }
    }, [handlePointerMove, handlePointerUp]);

    const formatTime = (number: number) => {
        let wholeSeconds = Math.round(number)
        const minutes = Math.floor(wholeSeconds / 60)
        wholeSeconds %= 60
        return `${minutes}:${wholeSeconds.toString().padStart(2, "0")}`
    }

    useEffect(() => {
        setTime([formatTime(getPosition()), formatTime(duration)])
        const pos = progressUpdating ? intermediatePosition : progress;

        setVars({ "--progress-bar-transform": `${pos * 100}%` } as React.CSSProperties)
    }, [getPosition, duration, progress, intermediatePosition, progressUpdating]);

    const handleMove = (e: PointerEvent) => {
        const bounds = parentElement.current?.getBoundingClientRect();
        if(bounds === undefined)
            return
        const x = Math.min((e.clientX - bounds.left), bounds.width);
        const newProgress = Math.max(0, x / bounds.width);
        setIntermediatePosition(newProgress)
    }

    return (
        <div
            className={`flex flex-row gap-x-5 outer-hover`}
        >
            <span className="w-[3%] text-zinc-400 text-sm select-none">{time[0]}</span>
            <div
                onPointerDown={e => {
                    setProgressUpdating(true)
                    handleMove(e as unknown as PointerEvent)
                }}
                className={`outer-hover h-[100%] w-[100%]`}
                ref={parentElement}
            >
                <div
                    className={`bg-zinc-600 rounded-full relative h-[5px] mb-4 w-[100%] mt-2 ${moreTailwind}`}
                    style={vars}
                >
                    <div className={`bg-zinc-50 h-[5px] rounded-full inner-hover`} style={{ width: `var(--progress-bar-transform)` }}></div>
                    <div className={"slider-pin h-[12px] w-[12px] shadow-black shadow mt-[-8px] rounded-[50%] ml-[-6px] bg-zinc-100 z-[100] absolute left-[var(--progress-bar-transform)]"}></div>
                </div>
            </div>
            <span className="text-zinc-400 text-sm select-none">{time[1]}</span>
        </div>
    )
}