import React, {useMemo, useState} from "react";
import {AlbumData} from "./AlbumView.tsx";
import {usePlaylistController} from "../../util/PlaylistController.tsx";
import EditIcon from "../icons/EditIcon.tsx";
import {PlayPause} from "../buttons/PlayButton.tsx";
import {Love} from "../icons/LoveIcon.tsx";
import {useBackendController} from "../../util/BackendController.tsx";
import UpdateIcon from "../icons/UpdateIcon.tsx";
import DeleteIcon from "../icons/DeleteIcon.tsx";
import {ViewType} from "./MainView.tsx";

export interface AlbumCoverDisplayProps {
    albumData: AlbumData,
    changeView: (v: ViewType) => void,
}

export const AlbumCoverDisplay: React.FC<AlbumCoverDisplayProps> = ({ albumData, changeView }) => {
    const { currentTrack } = usePlaylistController();
    const { post } = useBackendController();
    const [isEditing, setEditing] = useState(false);
    const [modAlbumName, setModAlbumName] = useState(albumData.name);

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
                onClick={() => setEditing(!isEditing)}
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
        {isEditing ? <div className={"flex flex-col gap-2 mt-0 justify-start"}>
            <span className={"flex flex-row self-center"}>
                <h3 className={"text-xl text-zinc-50 p-5"}>
                    Name:
                </h3>
                <input
                    className={"h-[2em] mt-4 p-2 w-[10em] bg-zinc-700 outline-none text-zinc-100 rounded-xl text-lg font-bold"}
                    type={"text"} defaultValue={albumData.name} onSelect={e => setModAlbumName(e.target.value)}/>
                {/* TODO: change artists, cover, etc. */}
            </span>
            <span className={"self-center flex flex-row pb-4 gap-2"}>
                <button className={"transition-all ease-in-out bg-zinc-700 hover:bg-zinc-600 rounded-xl w-[9em] flex flex-row p-2"}
                        onClick={async () => {
                            await post(`/upload/album/${albumData.id}/patch`, { album_name: modAlbumName }, 'PATCH').then(() => {
                                changeView(undefined);
                                setTimeout(() => changeView({album: albumData.id}), 1)

                            })
                        }}
                >
                    <UpdateIcon className={"scale-150 fill-zinc-50 ml-5 mt-1.5"}/>
                    <p className={"text-white text-lg ml-2"}>Update</p>
                </button>
                <button
                    className={"transition-all ease-in-out bg-zinc-700 hover:bg-red-400 rounded-xl w-[9em] flex flex-row p-2"}
                    onClick={async () => {
                        await post(`/upload/album/${albumData.id}/delete`, {}).then(() => {
                            console.log("Deleted album!");
                            changeView({search: undefined});
                        }).catch(err => {
                            console.error(err)
                        })
                    }}
                >
                    <DeleteIcon className={"scale-150 fill-zinc-50 ml-5 mt-1.5"}/>
                    <p className={"text-white text-lg ml-2"}>Delete</p>
                </button>
            </span>
        </div> : undefined}
    </div>

}