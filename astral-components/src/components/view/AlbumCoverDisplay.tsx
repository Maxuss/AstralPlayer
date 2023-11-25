import React, {useMemo} from "react";
import {AlbumData} from "./AlbumView.tsx";
import {usePlaylistController} from "../../util/PlaylistController.tsx";
import EditIcon from "../icons/EditIcon.tsx";
import {PlayPause} from "../buttons/PlayButton.tsx";
import {Love} from "../icons/LoveIcon.tsx";

export interface AlbumCoverDisplayProps {
    albumData: AlbumData
}

export const AlbumCoverDisplay: React.FC<AlbumCoverDisplayProps> = ({ albumData }) => {
    const { currentTrack } = usePlaylistController();

    const isPlayingAlbum = useMemo(() => {
        if(albumData === undefined)
            return false;
        const current = currentTrack()?.id || "";
        return albumData.tracks.find(any => any.id === current) !== undefined;
    }, [albumData, currentTrack])

    return <div className={"flex flex-col bg-[#3c3c3d40] rounded-xl h-fit"}>
        <img
            width={300}
            alt={`Album art for ${albumData?.name}`}
            src={`http://localhost:8080/metadata/album/${albumData.id}/cover`}
            className={"rounded-xl m-5 shadow-lg shadow-[#00000020]"}
        />
        <div className={"flex flex-row gap-5 mb-[10%] mt-5 justify-center"}>
            <EditIcon
                className="
                               fill-white hover:fill-zinc-200 transition-colors ease-in-out scale-[250%]
                               cursor-pointer mx-5"/>

            <PlayPause isPlaying={isPlayingAlbum}
                       className="
                               fill-white hover:fill-zinc-200 transition-colors ease-in-out scale-[300%]
                               cursor-pointer mx-5 "/>

            <Love love={false}
                  className="
                               fill-white hover:fill-zinc-200 transition-colors ease-in-out scale-[250%]
                               cursor-pointer mx-5"/>
        </div>
    </div>

}