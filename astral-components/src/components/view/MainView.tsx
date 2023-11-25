import {Navbar} from "../bar/Navbar.tsx";
import {AlbumView} from "./AlbumView.tsx";

export const MainView = () => {
    return <div className={"absolute w-[70%] left-[4%] right-[30%] h-[99%] top-[1%] bg-zinc-900 rounded-t-2xl overflow-hidden"}>
        <Navbar />
        <AlbumView albumId={"98420858-b6ee-4abe-a60a-19d15d8ebcac"} />
    </div>
}