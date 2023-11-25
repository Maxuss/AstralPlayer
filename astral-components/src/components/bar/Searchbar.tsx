import SearchIcon from "../icons/SearchIcon.tsx";
import './Searchbar.css'

export const Searchbar = () => {
    return <span className={"flex flex-row gap-5"}>
        <SearchIcon className={"scale-[150%] ml-3 mt-3 mr-[-0.5rem] fill-zinc-300"} />

        <input type={"text"} className={"searchbar bg-gradient-to-r from-zinc-800 to-transparent rounded-xl"} placeholder={"Search for song, artists, etc."} />

        <div className={"flex flex-row ml-[45%] gap-2"}>
            <div className={"w-8 h-8 rounded-full bg-zinc-200 animate-pulse"}></div>
            <div className={"text-zinc-200 mt-0.5 text-lg"}>maxus</div>
        </div>
    </span>
}