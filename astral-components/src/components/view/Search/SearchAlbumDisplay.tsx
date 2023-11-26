import {IndexedAlbum} from "./SearchView.tsx";
import React from "react";
import {ViewType} from "../MainView.tsx";

export interface AlbumDisplayProps {
    index: IndexedAlbum,
    setView: (view: ViewType) => void
}

export const SearchAlbumDisplay: React.FC<AlbumDisplayProps> = ({ index, setView }) => {
    const cutStr = (str: string) => {
        return str.length > 25 ? `${str.substring(0, 22)}...` : str
    }

    return <div className="
        select-none cursor-pointer transition-colors ease-in-out
        bg-[#2d2d2d63] rounded-xl
        w-[16rem] h-[20rem] mr-5
        flex flex-col overflow-clip
        group hover:bg-[#3d3d3d63]" onClick={() => setView({album: index.id})}
    >
        <img
            alt={`Album cover for ${index.name}`}
            src={`http://localhost:8080/metadata/album/${index.id}/cover`}
            className={"scale-[90%] group-hover:scale-100 transition-all pointer-events-none ease-in-out drop-shadow-[0_20px_13px_rgba(0,0,0,0.2)] group-hover:drop-shadow-lg rounded-lg"}
        />
        <span className={"font-montserrat text-white mx-5 group-hover:mt-2 transition-all ease-in-out"}>
            {cutStr(index.name)}
        </span>
        <span className={"text-zinc-400 text-sm font-light mx-5"}>
            <a className={"decoration-0 hover:underline"}>{cutStr(index.artists[0][1])}</a>
            &nbsp; • {index.release_date.substring(0, 4)}
        </span>
    </div>
}