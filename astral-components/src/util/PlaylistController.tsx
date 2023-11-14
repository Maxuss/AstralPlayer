/* eslint-disable react-hooks/rules-of-hooks */
import {MutableRefObject, useRef, useState} from "react";
import {useGlobalAudioPlayer} from "react-use-audio-player";
import {singletonHook} from "react-singleton-hook";

export type RepeatingKind = 'single' | 'collection' | 'disabled';

export interface PlaylistController {
    toggle: () => void,
    setVolume: (vol: number) => void,
    volume: number,
    next: () => void,
    back: () => void,
    goto: (pos: number) => void,
    getPosition: () => number,
    append: (track: QueuedTrack) => void,

    setShuffle: (state: boolean) => void,
    isShuffle: boolean,

    setRepeat: (state: RepeatingKind) => void,
    repeat: RepeatingKind,

    queue: MutableRefObject<QueuedTrack[]>,
    currentTrack: () => QueuedTrack | undefined,

    duration: number,
    isPlaying: boolean
}

export interface QueuedTrack {
    coverUrl: string,
    streamUrl: string,
    title: string,
    artist: string,
    album: string,
    format: "mp3" | "flac"
}

const useInitializePlaylistController: () => PlaylistController = () => {
    const [isShuffle, setShuffle] = useState(false)
    const [repeat, setRepeat] = useState<RepeatingKind>('disabled')

    const { seek, getPosition, load, togglePlayPause, volume, setVolume, stop, duration, playing } = useGlobalAudioPlayer()

    const queue = useRef<QueuedTrack[]>([])
    const [currentTrack, setCurrentTrack] = useState(-1)

    return {
        toggle: togglePlayPause,
        setVolume: setVolume,
        volume: volume,
        next: () => {
            let newTrack = currentTrack;
            if(repeat === "single") {
                // just skipping to the beginning
                seek(0)
                return
            } else if(repeat === "collection") {
                const queueLen = queue.current.length;
                if(queueLen === 0) {
                    // queue was emptied
                    setCurrentTrack(-1)
                    stop()
                    return
                } else {
                    newTrack = Math.min(0, currentTrack % queueLen)
                    setCurrentTrack(newTrack)
                }
            } else {
                // no repeating, so if we have reached the end of queue, set to -1
                const queueLen = queue.current.length;
                if(queueLen === 0 || currentTrack + 1 === queueLen) {
                    // queue was emptied / we have reached the end of queue
                    setCurrentTrack(-1)
                    stop()
                    return
                } else {
                    newTrack = currentTrack + 1
                    setCurrentTrack(currentTrack + 1)
                }
            }
            // we need to load the track now
            console.log(queue)
            console.log(newTrack)
            const track = queue.current[newTrack]
            stop()
            load(
                track.streamUrl,
                {
                    autoplay: true,
                    format: track.format
                }
            )
        },
        back: () => {
            if(getPosition() <= 6) {
                // less than 6 seconds -- skip to previous
                if(repeat === "single") {
                    seek(0)
                } else {
                    const queueLen = queue.current.length;
                    if(queueLen === 0) {
                        // queue was emptied
                        setCurrentTrack(-1)
                        stop()
                    } else if(currentTrack === 0 || currentTrack === -1) {
                        // return to the beginning
                        seek(0)
                        stop()
                    } else {
                        const newIdx = Math.max(0, currentTrack - 1)
                        setCurrentTrack(newIdx)
                        const track = queue.current[newIdx]
                        stop()
                        load(
                            track.streamUrl,
                            {
                                // autoplay: true,
                                format: track.format
                            }
                        )
                    }
                }
            } else {
                // just return to the beginning
                seek(0)
            }
        },
        goto: seek,
        getPosition: getPosition,
        append: track => {
            queue.current.push(track)
        },

        setShuffle: setShuffle,
        isShuffle: isShuffle,

        setRepeat: setRepeat,
        repeat: repeat,

        queue: queue,
        currentTrack: () => currentTrack === -1 ? undefined : queue.current[currentTrack],

        duration,
        isPlaying: playing
    }
}

export const usePlaylistController = singletonHook<PlaylistController>({
        toggle: () => { },
        setVolume: () => { },
        volume: 0,
        next: () => { },
        back: () => { },
        goto: () => { },
        getPosition: () => 0,
        append: () => { },
        setShuffle: () => { },
        isShuffle: false,
        setRepeat: () => { },
        repeat: 'disabled',
        queue: { current: [] },
        duration: 0,
        currentTrack: () => undefined,
        isPlaying: false
    },
    useInitializePlaylistController
)