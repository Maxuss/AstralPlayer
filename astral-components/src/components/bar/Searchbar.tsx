import SearchIcon from "../icons/SearchIcon.tsx";
import './Searchbar.css'

export const Searchbar = () => {
    return <span className={"flex flex-row gap-5 mt-2"}>
        <SearchIcon className={"scale-[150%] ml-3 mt-3 fill-zinc-300"} />

        <input type={"text"} className={"searchbar bg-gradient-to-r from-zinc-800 to-transparent rounded-xl"} placeholder={"Search for song, artists, etc."} />

        <div className={"flex flex-row absolute right-9 gap-2 select-none"}>
            <img src={"https://cdn.discordapp.com/avatars/381827687775207424/b8259800be4529e43408f6b340e08728?size=1024"} className={"w-8 h-8 rounded-full"} alt={"User avatar"} />
            <div className={"text-zinc-200 mt-0.5 text-lg"}>maxus</div>
        </div>
    </span>
}