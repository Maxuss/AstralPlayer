import {IndexedArtist} from "./SearchView.tsx";
import React from "react";
import {ViewType} from "../MainView.tsx";

export interface ArtistDisplayProps {
    index: IndexedArtist,
    setView: (view: ViewType) => void
}

export const SearchArtistDisplay: React.FC<ArtistDisplayProps> = ({ index, setView }) => {
    const cutStr = (str: string) => {
        return str.length > 25 ? `${str.substring(0, 21)}...` : str
    }

    return <div className="
        select-none cursor-pointer transition-colors ease-in-out
        bg-[#2d2d2d63] rounded-xl
        w-[20rem] h-[15rem] mr-5 mt-5
        flex flex-col
        group hover:bg-[#3d3d3d63]" onClick={() => { }}
    >
        <div className={"w-[5rem] h-[5rem] rounded-full animate-pulse bg-zinc-200 mx-5 mt-5"}></div>
        <div className={"font-montserrat text-3xl text-white ml-5 mt-3 transition-all ease-in-out w-[20rem]"}>
            {cutStr(index.name)}
        </div>
        <div className={"w-[5rem] h-[2rem] bg-neutral-900 ml-4 mt-[10em] rounded-3xl text-white text-xl pl-3 pt-0.5 absolute "}>
            Artist
        </div>
    </div>
}