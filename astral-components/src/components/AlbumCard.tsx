import React from "react";

/**
 * Properties for an album card
 */
interface AlbumCardProps {
    artUrl: string,

    name: string,
    artist: string,
    description: string
}

export const AlbumCard: React.FC<AlbumCardProps> = ({ artUrl, name, artist }: AlbumCardProps) => {
    return <div className="
    select-none cursor-pointer
    transition-all ease-linear delay-[50]
    rounded-md min-w-[10] sm:max-w-[12rem] md:max-w-[12rem] lg:max-w-[13rem] min-h-[20] flex-col flex-grow p-2
    bg-gradient-to-tl from-[#27083f] via-stone-900 to-stone-700
    bg-size-200 bg-pos-0 hover:bg-pos-100
    shadow-lg shadow-black
    hover:shadow-[#130321]
    ">
        <img width={"90%"} src={artUrl} className="rounded-md m-[5%] shadow-md shadow-neutral-900" alt={`Album art for ${name} by ${artist}`} />
        <p className="text-stone-50 font-bold text-left text-xl ml-[5%] mr-0 my-0 drop-shadow-md">{trimString(name, 15)}</p>
        <p className="text-stone-300 text-[90%] mx-[5%] mt-0 mb-[10%] drop-shadow-md hover:underline decoration-neutral-400">{trimString(artist, 20)}</p>
    </div>
}

function trimString(str: string, len: number) {
    return str.length > len ? `${str.slice(0, len)}...` : str;
}