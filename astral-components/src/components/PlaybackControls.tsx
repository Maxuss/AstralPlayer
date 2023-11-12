import React, {useEffect, useRef, useState} from "react";
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
    const progress = useRelativeAudioTime()

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
        <AudioProgressBar progress={progress} moreTailwind={""} />
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
    progress: number,
    moreTailwind: string
}

export const AudioProgressBar: React.FC<ProgressBarProps> = ({ progress, moreTailwind }: ProgressBarProps) => {
    const { getPosition, duration } = useGlobalAudioPlayer()
    const pos = getPosition()
    const [time, setTime] = useState(["0:00", "0:00"])
    const [vars, setVars] = useState({ "--progress-bar-transform": "0%" } as React.CSSProperties)


    useEffect(() => {
        setTime([formatTime(pos), formatTime(duration)])
        setVars({ "--progress-bar-transform": `${Math.round(progress * 100)}%` } as React.CSSProperties)
    }, [pos, duration, progress]);

    const formatTime = (number: number) => {
        let wholeSeconds = Math.round(number)
        const minutes = Math.floor(wholeSeconds / 60)
        wholeSeconds %= 60
        return `${minutes}:${wholeSeconds.toString().padStart(2, "0")}`
    }

    return (
        <div className={`flex flex-row gap-x-5`}>
            <span className="w-[3%] text-zinc-400 text-sm">{time[0]}</span>
            <div
                onClick={handlePlaybarClick}
                className={`bg-zinc-600 rounded-full relative   h-1 mb-4 w-[100%] outer-hover mt-2 ${moreTailwind}`}
                style={vars}>
                <div className={`bg-zinc-50 h-1 rounded-full inner-hover`} style={{ width: `var(--progress-bar-transform)` }}></div>
                <div className={"slider-pin h-[12px] w-[12px] shadow-black shadow mt-[-8px] rounded-[50%] ml-[-6px] bg-zinc-100 z-[100] absolute left-[var(--progress-bar-transform)]"}></div>
            </div>
            <span className="text-zinc-400 text-sm">{time[1]}</span>
        </div>
    )
}

function handlePlaybarClick(e: React.MouseEvent<HTMLDivElement>) {
    const bounds = e.target.getBoundingClientRect();
    const x = e.clientX - bounds.left;
    const progress = x / bounds.width;
    console.log(progress)
}