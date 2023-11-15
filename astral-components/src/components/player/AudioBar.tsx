import React, {useCallback, useEffect, useRef, useState} from "react";
import {useGlobalAudioPlayer} from "react-use-audio-player";
import {usePlaylistController} from "../../util/PlaylistController.tsx";
import './AudioBar.css'

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

export const AudioBar = () => {
    const { getPosition, goto, duration } = usePlaylistController();
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

    useEffect(() => {
        let pos = progressUpdating ? intermediatePosition : progress;
        pos = Number.isNaN(pos) || !    Number.isFinite(pos) ? 0 : pos;

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
            className={`absolute bottom-0 top-0 left-0 right-0 m-0 w-full rounded-xl`}
        >
            <div
                onPointerDown={e => {
                    setProgressUpdating(true)
                    handleMove(e as unknown as PointerEvent)
                }}
                className={`w-[100%] outer-hover`}
                ref={parentElement}
            >
                <div
                    className={`bg-zinc-600 relative rounded-t-lg mb-4 w-[100%]`}
                    style={vars}
                >
                    <div className={`bg-zinc-50 h-[5px] rounded-t inner-hover`} style={{ width: `var(--progress-bar-transform)` }}></div>
                    <div className={"slider-pin h-[12px] w-[12px] shadow-black shadow mt-[-8px] rounded-[50%] ml-[-6px] bg-zinc-100 z-[100] absolute left-[var(--progress-bar-transform)]"}></div>
                </div>
            </div>
        </div>
    )
}