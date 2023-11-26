import React, {useEffect, useState} from "react";
import './AlbumView.css'
import {CircleIcon} from "../icons/CircleIcon.tsx";
import {AlbumTrack} from "./AlbumTrack.tsx";
import {AlbumCoverDisplay} from "./AlbumCoverDisplay.tsx";
import {useBackendController} from "../../util/BackendController.tsx";

export interface AlbumViewProps {
    albumId: string
}

export interface AlbumTrackData {
    index: number,
    id: string,
    name: string,
    length: number,
    format: string,
    artist: string,
}

export interface AlbumData {
    id: string,
    name: string,
    artist: string,
    releaseDate: string,
    yourRating: number,
    rymRating: number,
    tracks: AlbumTrackData[]
}

export const AlbumView: React.FC<AlbumViewProps> = ({ albumId }) => {
    const [albumData, setAlbumData] = useState<AlbumData | undefined>({
        id: albumId,
        name: "JOECHILLWORLD",
        artist: "Devon Hendryx",
        releaseDate: "2011",
        yourRating: 5.0,
        rymRating: 3.8,
        tracks: []
    });
    const [lastFetched, setLastFetched] = useState<string | undefined>();
    const { get } = useBackendController();

    useEffect(() => {
        if(lastFetched === albumId)
            return;

        get(`/metadata/album/${albumId}`).then(async metadata => {
            const extractedTracks = [];
            const resolvedArtists = {}

            await Promise.all(metadata.metadata.tracks.map(async track => {
                let artistString = "";
                for(const artistId of track.artist_ids) {
                    if(artistId in resolvedArtists) {
                        artistString += resolvedArtists[artistId];
                        artistString += ", ";
                    } else {
                        const name =
                            await get(`/metadata/artist/${artistId}`).then(artist => artist.metadata.artist_name);
                        artistString += name;
                        artistString += ", ";
                        resolvedArtists[artistId] = name;
                    }
                }
                extractedTracks.push({
                    index: track.number,
                    id: track.track_id,
                    name: track.track_name,
                    length: track.track_length,
                    format: track.format,
                    artist: artistString.substring(0, artistString.length - 2)
                })
            }))

            const meta = metadata.metadata;
            const data = {
                id: metadata.album_id,
                artist: meta.artists.map(each => each.artist_name).join(", "),
                name: meta.album_name,
                releaseDate: meta.release_date.substring(0, 4),
                yourRating: 4.5,
                rymRating: 4.5,
                tracks: extractedTracks,
            };
            data.tracks.sort((a, b) => a.index < b.index ? -1 : a.index > b.index ? 1 : 0)
            setAlbumData(data)
            setLastFetched(albumId);
        }).catch(err => console.error("Failed to fetch album metadata.", err))

    }, [albumId, get]);

    function setClasses(el) {
        const isScrollable = el.scrollHeight > el.clientHeight;

        if (!isScrollable) {
            el.classList.remove('is-bottom-overflowing', 'is-top-overflowing');
            return;
        }
        const isScrolledToBottom = el.scrollHeight < el.clientHeight + el.scrollTop + 1;
        const isScrolledToTop = isScrolledToBottom ? false : el.scrollTop === 0;
        el.classList.toggle('is-bottom-overflowing', !isScrolledToBottom);
        el.classList.toggle('is-top-overflowing', !isScrolledToTop);
    }

    return <>
        <img alt={""} className={"album-backdrop pointer-events-none object-fill blur-3xl opacity-[10%] w-full overflow-hidden"} src={`http://localhost:8080/metadata/album/${albumId}/cover`} />

        <div className={"flex flex-row flex-grow absolute left-[10%] top-[20%] select-none"}>
            <AlbumCoverDisplay albumData={albumData!} />
            <div className={"ml-5 flex flex-col"}>
                <span className={"flex flex-row w-[100%]"}>
                    <span className={"text-5xl font-extrabold text-zinc-100 font-montserrat w-[35rem] max-h-[30rem]"}>
                        {albumData?.name}
                    </span>
                    <span className={"text-4xl mt-2 font-extrabold text-zinc-100 font-montserrat absolute right-0"}>
                        {albumData?.rymRating}
                        <span className={"text-lg"}>
                            /5
                        </span>
                    </span>
                </span>
                <span className={"flex flex-row w-[100%]"}>
                    <span className={"text-sm text-zinc-300 w-[200%] flex-grow"}>
                        {albumData?.artist} &nbsp;• &nbsp;{albumData?.releaseDate} • &nbsp;{albumData?.tracks?.length} Songs
                    </span>
                    <span className={"flex flex-row absolute ml-[56%]"}>
                        <CircleIcon style={"full"} className={"fill-white scale-[80%] hover:fill-zinc-200 transition-colors ease-in-out cursor-pointer"} />
                        <CircleIcon style={"full"} className={"fill-white scale-[80%] hover:fill-zinc-200 transition-colors ease-in-out cursor-pointer"} />
                        <CircleIcon style={"full"} className={"fill-white scale-[80%] hover:fill-zinc-200 transition-colors ease-in-out cursor-pointer"} />
                        <CircleIcon style={"full"} className={"fill-white scale-[80%] hover:fill-zinc-200 transition-colors ease-in-out cursor-pointer"} />
                        <CircleIcon style={"half"} className={"fill-white scale-[80%] hover:fill-zinc-200 transition-colors ease-in-out cursor-pointer"} />
                    </span>
                </span>
                <div id={"scrollable-wrapper"} onScroll={e => {
                    const el = e.currentTarget;
                    setClasses(el);
                }} className={"is-bottom-overflowing flex flex-col mt-10 py-2 w-[40rem] gap-1 max-h-[70%] overflow-y-scroll"}>
                    {
                        albumData?.tracks.sort().map(each => (
                            <AlbumTrack key={each.index} data={each} album={albumData?.name || ""}/>
                        ))
                    }
                </div>
            </div>

        </div>
    </>
}