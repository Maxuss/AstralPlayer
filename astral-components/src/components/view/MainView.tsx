import {Navbar} from "../bar/Navbar.tsx";
import {AlbumView} from "./AlbumView.tsx";
import {SearchView} from "./Search/SearchView.tsx";
import React, {createRef, useCallback, useState} from "react";

export type ViewType = { album: string } | { search: string | undefined } | undefined;

export const MainView = () => {
    const [viewType, setViewType] = useState<ViewType>({ search: undefined })
    const parentDiv = createRef<HTMLDivElement>()
    const changeView = useCallback((view: ViewType) => {
        parentDiv.current!.scrollTop = 0;
        setViewType(view);
    }, [parentDiv])

    return <div
        ref={parentDiv}
        style={{
            overflowY: typeof viewType === "object" && "search" in viewType ? "scroll" : "hidden"
        }}
        className={`absolute w-[70%] left-[4%] right-[30%] h-[99%] top-[1%] bg-zinc-900 rounded-t-2xl overflow-x-hidden`}
    >
        <Navbar setSearch={(v) => v.length === 0 ? setViewType({ search: undefined }) : setViewType({ search: v })} />

        {viewType === undefined ? <div></div> : "album" in viewType ? <AlbumView albumId={viewType.album} /> : <SearchView setView={changeView} search={viewType.search} />}
    </div>
}