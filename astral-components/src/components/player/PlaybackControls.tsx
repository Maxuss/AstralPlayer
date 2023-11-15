import React, {useEffect} from "react";
import {PlayButton} from "../buttons/PlayButton.tsx";
import PauseButton from "../buttons/PauseButton.tsx";
import ShuffleButton from "../buttons/ShuffleButton.tsx";
import RepeatButton from "../buttons/RepeatButton.tsx";
import BackwardButton from "../buttons/BackwardButton.tsx";
import ForwardButton from "../buttons/ForwardButton.tsx";
import {usePlaylistController} from "../../util/PlaylistController.tsx";

interface PlayPauseProps {
    isPlaying: boolean,
}

const PlayPause: React.FC<PlayPauseProps & React.SVGProps<SVGSVGElement>> = ({ isPlaying, ... props }) => {
    return isPlaying ? <PauseButton {...props} /> : <PlayButton {...props} />
}

export const PlaybackControls = () => {
    const { toggle, repeat, setRepeat, isShuffle, setShuffle, next, back, queue, isPlaying } = usePlaylistController();

    useEffect(() => {
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
    </div>)
}
