/* eslint-disable react-hooks/rules-of-hooks */
import {MutableRefObject, useCallback, useRef, useState} from "react";
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
    title: string,
    artist: string,
    album: string,
    format: "mp3" | "flac",
    id: string | undefined,
}

export const coverUrl = (track: QueuedTrack | undefined): string => {
    return `${COVER_URL}${track?.id || "-"}/cover`
}

const COVER_URL = "http://localhost:8080/metadata/track/"
const STREAM_URL = "http://localhost:8080/stream/"

const useInitializePlaylistController: () => PlaylistController = () => {
    const [isShuffle, setShuffle] = useState(false)
    const [repeat, setRepeat] = useState<RepeatingKind>('disabled')
    const [volumeState, setVolumeState] = useState(1)

    const { seek, getPosition, load, togglePlayPause, setVolume, stop, duration, playing } = useGlobalAudioPlayer()

    const queue = useRef<QueuedTrack[]>([])
    const [currentTrack, setCurrentTrack] = useState(-1)

    const next = useCallback(() => {
        let newTrack = -1;
        setCurrentTrack(trackNum => {
            newTrack = trackNum;
            if(repeat === "single") {
                // just skipping to the beginning
                const track = queue.current[Math.max(0, trackNum)];
                load(
                    `${STREAM_URL}${track.id}`,
                    {
                        initialVolume: volumeState,
                        autoplay: true,
                        format: track.format,
                        html5: true,
                        onend: next
                    }
                )
                return trackNum
            } else if(repeat === "collection") {
                const queueLen = queue.current.length;
                if(queueLen === 0) {
                    // queue was emptied
                    stop();
                    newTrack = -1;
                    return -1
                } else {
                    newTrack = Math.max(0, (trackNum + 1) % queueLen)
                    const track = queue.current[Math.max(0, newTrack)];
                    load(
                        `${STREAM_URL}${track.id}`,
                        {
                            initialVolume: volumeState,
                            autoplay: true,
                            format: track.format,
                            html5: true,
                            onend: next
                        }
                    )
                    return newTrack
                }
            } else {
                // no repeating, so if we have reached the end of queue, set to -1
                const queueLen = queue.current.length;
                if(queueLen === 0 || trackNum + 1 === queueLen) {
                    // queue was emptied / we have reached the end of queue
                    stop();
                    return -1
                } else {
                    newTrack = trackNum + 1;
                    const track = queue.current[Math.max(0, newTrack)];
                    load(
                        `${STREAM_URL}${track.id}`,
                        {
                            initialVolume: volumeState,
                            autoplay: true,
                            format: track.format,
                            html5: true,
                            onend: next,
                        }
                    )

                    return newTrack
                }
            }
        })
    }, [load, repeat, stop, volumeState])

    const back = useCallback(() => {
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
                        `${STREAM_URL}${track.id}`,
                        {
                            autoplay: true,
                            format: track.format,
                            html5: true,
                            initialVolume: volumeState,
                            onend: next
                        }
                    )
                }
            }
        } else {
            // just return to the beginning
            seek(0)
        }
    }, [currentTrack, getPosition, load, next, repeat, seek, stop, volumeState]);

    return {
        toggle: togglePlayPause,
        setVolume: vol => {
            setVolumeState(vol)
            setVolume(vol)
        },
        volume: volumeState,
        next: next,
        back: back,
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