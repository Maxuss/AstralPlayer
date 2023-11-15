import {usePlaylistController} from "../../util/PlaylistController.tsx";
import React, {useCallback, useEffect, useRef, useState} from "react";
import './VolumeBar.css'
import {IconVolumeLow, IconVolumeOff} from "../icons/VolumeIcons.tsx";

export const VolumeBar = () => {
    const { volume, setVolume } = usePlaylistController()
    const [volumeUpdating, setVolumeUpdating] = useState(false)
    const [vars, setVars] = useState({ "--volume-bar-transform": "0%" } as React.CSSProperties)
    const parentElement = useRef<HTMLDivElement>(null)

    const handlePointerMove = useCallback((e: PointerEvent) => {
        if(volumeUpdating)
            handleMove(e)
    }, [volumeUpdating])

    const handlePointerUp = useCallback((e: PointerEvent) => {
        if(volumeUpdating) {
            setVolumeUpdating(false)
            handleMove(e)
        }
    }, [volumeUpdating, setVolumeUpdating])

    const handleMove = (e: PointerEvent) => {
        const bounds = parentElement.current?.getBoundingClientRect();
        if(bounds === undefined)
            return
        const x = Math.min((e.clientX - bounds.left), bounds.width);
        const newVolume = Math.max(0, x / bounds.width);
        setVolume(newVolume)
    }

    useEffect(() => {
        window.addEventListener("pointermove", handlePointerMove)
        window.addEventListener("pointerup", handlePointerUp)

        return () => {
            window.removeEventListener("pointermove", handlePointerMove)
            window.removeEventListener("pointerup", handlePointerUp)
        }
    }, [handlePointerMove, handlePointerUp]);

    useEffect(() => {
        setVars({ "--volume-bar-transform": `${volume * 100}%` } as React.CSSProperties)
    }, [volume, setVars])

    return (
        <div className={"flex flex-row"}>
            {volume < 0.01 ?
                (
                    <IconVolumeOff className={"fill-zinc-300 scale-[120%] mr-2 mt-2"} />
                ) :
                (
                    <IconVolumeLow className={"fill-zinc-300 scale-[120%] mr-2 mt-2"} />
                )
            }
            <div
                onPointerDown={e => {
                    setVolumeUpdating(true)
                    handleMove(e as unknown as PointerEvent)
                }}
                className={`w-[100%] volume-outer-hover mt-[0.85rem]`}
                ref={parentElement}
            >
                <div
                    className={`bg-zinc-600 relative rounded-full mb-4 w-[100%]`}
                    style={vars}
                >
                    <div className={`bg-zinc-50 h-[4px] rounded-full volume-inner-hover`} style={{ width: `var(--volume-bar-transform)` }}></div>
                    <div className={"volume-pin h-[12px] w-[12px] shadow-black shadow mt-[-8px] rounded-[50%] ml-[-6px] bg-zinc-100 z-[100] absolute left-[var(--volume-bar-transform)]"}></div>
                </div>
            </div>
        </div>
    )
}