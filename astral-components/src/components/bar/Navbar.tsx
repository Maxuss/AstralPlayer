import {Searchbar} from "./Searchbar.tsx";
import React from "react";

export interface NavbarProps {
    setSearch: (s: string) => void
}

export const Navbar: React.FC<NavbarProps> = ({ setSearch }) => {
    return <div className={"w-full sticky top-0 z-10 bg-zinc-900 py-2"}>
        <Searchbar setSearch={setSearch} />
    </div>
}