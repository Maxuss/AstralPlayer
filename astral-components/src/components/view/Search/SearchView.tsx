import React, {useEffect, useState} from "react";
import {useBackendController} from "../../../util/BackendController.tsx";
import {SearchAlbumDisplay} from "./SearchAlbumDisplay.tsx";
import './SearchView.css'
import {ViewType} from "../MainView.tsx";
import {SearchArtistDisplay} from "./SearchArtistDisplay.tsx";

export interface SearchProps {
    search: string | undefined,
    setView: (view: ViewType) => void
}

export interface IndexedAlbum {
    id: string,
    name: string,
    artists: [string, string][],
    tracks: [string],
    release_date: string,
    genres: string[],
    loved: boolean
}

export interface IndexedArtist {
    id: string,
    name: string,
}

export interface IndexedTrack {
    id: string,
    name: string,
    album_id: string,
    album_name: string,
    artists: [string, string][],
    duration: number,
    format: 'mp3' | 'flac' | 'm4a',
    loved: boolean
}

export const SearchView: React.FC<SearchProps> = ({ search, setView }) => {
    const [albums, setAlbums] = useState<IndexedAlbum[]>([])
    const [artists, setArtists] = useState<IndexedArtist[]>([])
    const [tracks, setTracks] = useState<IndexedTrack[]>([])

    const { get, loading } = useBackendController();

    useEffect(() => {
        if(loading)
            return;
        const searchParams = `&skip=0${search === undefined ? "" : `&search=${encodeURIComponent(search)}`}`
        get(`/index/tracks?count=4${searchParams}`).then(res => {
            setTracks(res);
        }).catch(err => {
            console.error("Failed to index tracks", err);
        })
        get(`/index/albums?count=4${searchParams}`).then(res => {
            setAlbums(res);
        }).catch(err => {
            console.error("Failed to index albums", err);
        })
        get(`/index/artists?count=6${searchParams}`).then(res => {
            setArtists(res);
        }).catch(err => {
            console.error("Failed to index artists", err);
        })
    }, [search, get, loading]);

    return <>
        <h2 className={"text-4xl text-zinc-200 font-bold font-montserrat mt-[1%] ml-[1.6em] select-none"}>
            Albums
        </h2>
        <div className={"inline-grid md:grid-rows grid-flow-col display-scroll mx-[3.2em] mt-5"}>
            {
                albums?.map((each, idx) => (
                    <SearchAlbumDisplay key={idx} setView={setView} index={each} />
                ))
            }
        </div>
        <h2 className={"text-4xl text-zinc-200 font-bold font-montserrat mt-[6%] ml-[1.6em] select-none"}>
            Artists
        </h2>
        <div className={"inline-grid grid-rows-2 grid-flow-col display-scroll mx-[3.2em] mt-5"}>
            {
                artists?.map((each, idx) => (
                    <SearchArtistDisplay key={idx} setView={setView} index={each} />
                ))
            }
        </div>

        <h2 className={"text-4xl text-zinc-200 font-bold font-montserrat mt-[6%] ml-[1.6em] select-none"}>
            Songs
        </h2>
        <div className={"inline-grid md:grid-rows grid-flow-col display-scroll mx-[3.2em] mt-5"}>
        </div>

    </>
}