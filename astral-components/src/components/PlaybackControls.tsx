import React, {useCallback, useEffect, useRef, useState} from "react";
import {PlayButton} from "./buttons/PlayButton.tsx";
import PauseButton from "./buttons/PauseButton.tsx";
import {useGlobalAudioPlayer} from "react-use-audio-player";
import Track1 from '../../audio/Sakura.mp3';
import ShuffleButton from "./buttons/ShuffleButton.tsx";
import RepeatButton from "./buttons/RepeatButton.tsx";
import BackwardButton from "./buttons/BackwardButton.tsx";
import ForwardButton from "./buttons/ForwardButton.tsx";
import './PlaybackControls.css'

interface PlayPauseProps {
    isPlaying: boolean,
}

const PlayPause: React.FC<PlayPauseProps & React.SVGProps<SVGSVGElement>> = ({ isPlaying, ... props }) => {
    return isPlaying ? <PauseButton {...props} /> : <PlayButton {...props} />
}

export const PlaybackControls = () => {
    const { load, togglePlayPause } = useGlobalAudioPlayer()
    const [isShuffle, setShuffle] = useState(true)
    const [repeatType, setRepeatType] = useState<'single' | 'collection' | 'none'>('none')

    useEffect(() => {
        load(Track1)
    }, [load])

    const [isPlaying, setPlaying] = useState(false);

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
            />
            <PlayPause
                isPlaying={isPlaying}
                className="fill-white hover:fill-violet-200 transition-colors ease-in-out scale-[250%] cursor-pointer"
                onClick={() => { setPlaying(!isPlaying); togglePlayPause() }}
            />
            <ForwardButton
                className="fill-zinc-400 hover:fill-violet-200 transition-colors ease-in-out scale-[150%] cursor-pointer"
            />
            <RepeatButton
                repeat={repeatType === 'none' ? 'collection' : repeatType}
                className={`
                    ${repeatType !== 'none' ? "fill-[#9573f4] hover:fill-violet-400" : "fill-zinc-400 hover:fill-violet-200"} 
                    transition-colors ease-in-out scale-[160%] cursor-pointer
                `}
                onClick={() => {
                    if(repeatType === 'none')
                        setRepeatType('collection')
                    else if(repeatType === 'collection')
                        setRepeatType('single')
                    else
                        setRepeatType('none')
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
    const { getPosition, seek, duration } = useGlobalAudioPlayer()
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
        console.log(`CLICKING ${progressUpdating}`)
        if(progressUpdating) {
            setProgressUpdating(false)
            handleMove(e)
            seek(intermediatePosition * duration)
        }
    }, [progressUpdating, seek, intermediatePosition, duration, setProgressUpdating])

    useEffect(() => {
        window.addEventListener("pointermove", handlePointerMove)
        window.addEventListener("pointerup", handlePointerUp)

        return () => {
            // i have spent over 4 hours because of not handling unmounting
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
        const newProgress = x / bounds.width;
        setIntermediatePosition(newProgress)
    }

    return (
        <div className={`flex flex-row gap-x-5`}>
            <span className="w-[3%] text-zinc-400 text-sm select-none">{time[0]}</span>
            <div
                onPointerDown={e => {
                    setProgressUpdating(true)
                    handleMove(e as unknown as PointerEvent)
                }}
                className={`bg-zinc-600 rounded-full relative h-1 mb-4 w-[100%] outer-hover mt-2 ${moreTailwind}`}
                style={vars}
                ref={parentElement}
            >
                <div className={`bg-zinc-50 h-1 rounded-full inner-hover`} style={{ width: `var(--progress-bar-transform)` }}></div>
                <div className={"slider-pin h-[12px] w-[12px] shadow-black shadow mt-[-8px] rounded-[50%] ml-[-6px] bg-zinc-100 z-[100] absolute left-[var(--progress-bar-transform)]"}></div>
            </div>
            <span className="text-zinc-400 text-sm select-none">{time[1]}</span>
        </div>
    )
}