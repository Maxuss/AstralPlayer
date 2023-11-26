import {Searchbar} from "./Searchbar.tsx";

export const Navbar = () => {
    return <div className={"w-full sticky top-0 z-10 bg-zinc-900 py-2"}>
        <Searchbar />
    </div>
}