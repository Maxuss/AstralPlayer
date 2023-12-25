import {Searchbar} from "./Searchbar.tsx";
import React from "react";
import UploadIcon from "../icons/UploadIcon.tsx";
import {ViewType} from "../view/MainView.tsx";

export interface NavbarProps {
    setSearch: (s: string) => void,
    setView: (v: ViewType) => void
}

export const Navbar: React.FC<NavbarProps> = ({ setSearch, setView }) => {
    return <div className={"w-full sticky top-0 z-10 bg-zinc-900 py-2 flex flex-row"}>
        <Searchbar setSearch={setSearch}/>

        <UploadButton setView={setView} />

        {/* TODO: proper user page */}
        <div className={"flex flex-row absolute right-9 gap-2 select-none mt-2"}>
            <img
                src={"https://images-ext-2.discordapp.net/external/0wBRZzBXdLGPA48pl0USxNpi4_uM2_ARzNg3bE9phK8/%3Fsize%3D2048/https/cdn.discordapp.com/avatars/381827687775207424/fe994476bd26ae1775cb412fc1c7223a.png"}
                className={"w-8 h-8 rounded-full"} alt={"User avatar"}/>
            <div className={"text-zinc-200 mt-0.5 text-lg"}>maxus</div>
        </div>

    </div>
}

interface UploadButtonProps {
    setView: (v: ViewType) => void
}

const UploadButton: React.FC<UploadButtonProps> = ({ setView }) => {
    return <div className={"mt-2"}>
        <button
            className={"transition-all ease-in-out duration-500 rounded-full bg-zinc-800 hover:bg-zinc-400 w-[7em] h-10 flex flex-row"}
            onClick={() => setView("upload")}
        >
            <UploadIcon className={"fill-zinc-50 scale-[150%] min-w-fit ml-[15%] mt-2.5"} />
            <p className={"text-md font-bold text-zinc-50 mt-[0.4em] ml-[0.5em]"}>
                Upload
            </p>
        </button>
    </div>
}