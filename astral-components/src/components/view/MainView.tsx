import {Navbar} from "../bar/Navbar.tsx";
import {AlbumView} from "./AlbumView.tsx";
import {SearchView} from "./Search/SearchView.tsx";
import {createRef, useCallback, useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import {extractCookie} from "../../util/BackendController.tsx";
import {UploadView} from "./Upload/UploadView.tsx";

export type ViewType = { album: string } | { search: string | undefined } | 'upload' | undefined;

export const MainView = () => {
    const [viewType, setViewType] = useState<ViewType>({ search: undefined })
    const parentDiv = createRef<HTMLDivElement>()
    const changeView = useCallback((view: ViewType) => {
        parentDiv.current!.scrollTop = 0;
        setViewType(view);
    }, [parentDiv])

    const path = window.location.pathname
    const navigate = useNavigate();
    useEffect(() => {
        const loggedIn = extractCookie("refresh-token")
        if (loggedIn === "")
            navigate(`/auth?then=${path}`)
    }, [])

    return <div
        ref={parentDiv}
        style={{
            overflowY: viewType === 'upload' || (typeof viewType === "object" && "search" in viewType) ? "scroll" : "hidden"
        }}
        className={`absolute w-[69.5%] left-[2.5%] right-[30%] h-[87%] top-[1%] bg-zinc-900 rounded-t-2xl rounded-b-lg overflow-x-hidden display-scroll`}
    >
        <Navbar setView={setViewType} setSearch={(v) => v.length === 0 ? setViewType({ search: undefined }) : setViewType({ search: v })} />

        {viewType === undefined ? <div></div> : viewType === 'upload' ? <UploadView /> : "album" in viewType ? <AlbumView albumId={viewType.album} /> : <SearchView setView={changeView} search={viewType.search} />}
    </div>
}