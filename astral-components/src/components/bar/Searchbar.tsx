import SearchIcon from "../icons/SearchIcon.tsx";
import './Searchbar.css'
import React from "react";

export interface SearchbarProps {
    setSearch: (s: string) => void
}

export const Searchbar: React.FC<SearchbarProps> = ({ setSearch }) => {
    return <span className={"flex flex-row gap-5 mt-2"}>
        <SearchIcon className={"scale-[150%] ml-5 mt-3 fill-zinc-300"} />

        <input type={"text"} onInput={e => setSearch(e.target.value)} className={"searchbar bg-gradient-to-r from-zinc-800 to-transparent rounded-xl"} placeholder={"Search for song, artists, etc."} />

    </span>
}