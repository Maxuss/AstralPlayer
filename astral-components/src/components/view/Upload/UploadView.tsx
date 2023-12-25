import {useCallback, useEffect, useMemo, useState} from "react";
import * as metadata from "music-metadata-browser";
import {Buffer} from "buffer";
import * as process from "process";
import {musixmatchSearch, MusixmatchTrackData} from "../../../util/MusixmatchHandler.tsx";
import {useBackendController} from "../../../util/BackendController.tsx";

async function fetchFileData(file: File, get: (path: string) => Promise<unknown>): Promise<MusixmatchTrackData> {
    const parsed = await metadata.parseReadableStream(file.stream(), { size: file.size, mimeType: file.name }).catch(() => undefined);
    if (parsed === undefined) {
        return {
            name: file.name,
            album: "unknown",
            artist: "unknown",
            duration: 0,
            coverUrl: "none",
            releaseDate: new Date().toDateString()
        }
    }
    const meta = parsed.common;
    return await musixmatchSearch(meta.title, meta.album, meta.artist, undefined, get).then(data => {
        // we prefer actual file duration
        if(data !== undefined) {
            data.duration = Math.round(parsed.format.duration || data.duration);
        } else {
            throw new Error("oops") // this is just to exit, it's okay
        }
        return data
    }).catch(() => {
        const picture = parsed.common.picture === undefined ? "none" : (window.URL || window.webkitURL).createObjectURL(new Blob([metadata.selectCover(parsed.common.picture)!.data]));
        const date = parsed.common.date || `${parsed.common.year}-01-01`;
        return {
            name: parsed.common.title || file.name,
            album: parsed.common.album || "unknown",
            artist: parsed.common.artist || "unknown",
            duration: Math.round(parsed.format.duration || 0),
            coverUrl: picture,
            releaseDate: date
        } as MusixmatchTrackData;
    })
}


export const UploadView = () => {
    const [files, setFiles] = useState<FileList | undefined>()
    const {get, post, postFile} = useBackendController();

    useEffect(() => {
        // this is cursed tbh
        window.Buffer = Buffer;
        window.process = process;
    }, []);

    const extractedFiles = useMemo(() => {
        if (files === undefined)
            return undefined;
        const promises = []
        for (let i = 0; i < files.length; i++) {
            const file = files.item(i)!;
            promises.push(fetchFileData(file, get));
        }
        return promises
    }, [files, get])

    const uploadToServer = useCallback(() => {
        if(files === undefined || extractedFiles === undefined)
            return;
        for(let i = 0; i < files.length; i++) {
            const fileData = files.item(i)!;
            extractedFiles[i].then(async file => {
                const splitName = fileData.name.split(".");
                const ext = splitName[splitName.length - 1];
                postFile(`/upload/track/${ext}`, (await fileData.arrayBuffer())).then(async resp => {
                    const trackId = resp.track_id;
                    // TODO: uploading other edited info here
                    await post(`/upload/guess_metadata/${trackId}?musix_priority=true&musix_artist_override=${file.artist}&musix_album_override=${file.album}&musix_name_override=${file.name}`, {}).then(async innerResp => {
                        if(innerResp === undefined) {
                            // failed to upload with musix, skipping musix then
                            await post(`/upload/guess_metadata/${trackId}?skip_musix=true&musix_priority=true&musix_artist_override=${file.artist}&musix_album_override=${file.album}&musix_name_override=${file.name}`, {}).then(() => {
                                console.log("Finished uploading track")
                            })
                        } else {
                            console.log(`Uploaded track ${trackId}`)
                        }
                    }).catch(async err => {
                        console.error("Failed to upload track metadata")
                        console.error(err)
                    })
                });
            });
        }
    }, [extractedFiles, files])

    return <>
        <div className={"mt-[1%] flex flex-col"}>
            <h1 className={"ml-[1.6em] text-4xl text-zinc-50 font-montserrat "}>
                Upload music to the server
            </h1>
            <p className={"ml-[3.2em] text-lg text-zinc-100 mt-2"}>
                Drag and drop your files here or select them to upload them to the server
            </p>

            {/* TODO: improve this input */}
            <input type="file" name="fileUpload" accept={"audio/flac, audio/mpeg, audio/mp4"} multiple={true}
                   onChange={e => setFiles(e.target.files as FileList)}/>

            <div className={"self-center grid grid-cols-5 gap-5"}>
                {
                    extractedFiles?.map(((each, idx) => <PreuploadedCard promise={each}
                                                                         key={files?.item(idx)?.name || "unknown"}/>))
                }
            </div>


            <button
                className={"mt-5 self-center bg-zinc-700 outline outline-2 outline-transparent hover:bg-zinc-500 hover:outline-zinc-50 transition-all ease-in-out rounded-lg text-xl  text-zinc-50 w-[20%] h-10"}
                onClick={uploadToServer}
                >
                Upload to server
            </button>
        </div>
    </>
}

const PreuploadedCard = ({promise}: { promise: Promise<MusixmatchTrackData> }) => {
    const [track, setTrack] = useState<MusixmatchTrackData | undefined>()

    useEffect(() => {
        promise.then(setTrack)
    }, []);

    return <>
    {
            track === undefined ? <LoadingCard /> :
                <div
                    className={"rounded-lg transition-all ease-in-out bg-[#2d2d2d63] hover:bg-[#3d3d3d63] w-[14em] h-[22.5em] flex flex-col"}>
                    <p className={"text-zinc-100 text-lg mt-5 ml-[1.5em] absolute rounded-tl-xl rounded-br-xl w-[3em] pl-2 bg-zinc-900 select-none"}>
                        {Math.round(track.duration / 60)}:{(track.duration % 60).toString().padStart(2, "0")}
                    </p>

                    <img alt={"Cover art"} src={track.coverUrl} className={"rounded-xl self-center mt-5"} width={"75%"}
                         height={"75%"}/>
                    <input
                        className={"bg-zinc-800 outline outline-2 outline-zinc-700 self-center mt-2 text-zinc-50 px-2 w-[75%] rounded-lg text-lg"}
                        defaultValue={track.name} placeholder={"Song name"}/>
                    <input
                        className={"bg-zinc-800 outline outline-2 outline-zinc-700 self-center mt-2 text-zinc-50 px-2 w-[75%] rounded-lg text-lg"}
                        defaultValue={track.album} placeholder={"Album name"}/>
                    <input
                        className={"bg-zinc-800 outline outline-2 outline-zinc-700 self-center mt-2 text-zinc-50 px-2 w-[75%] rounded-lg text-lg"}
                        defaultValue={track.artist} placeholder={"Artist"}/>
                    <input
                        className={"bg-zinc-800 outline outline-2 outline-zinc-700 self-center mt-2 text-zinc-50 px-2 w-[75%] rounded-lg text-lg"}
                        defaultValue={track.releaseDate.substring(0, 10)} placeholder={"Release date"} type={"date"}/>
                    {/*  TODO: reload button  */}
                </div>
        }
    </>
}

const LoadingCard = () => {
    return <div className={"rounded-lg bg-[#2d2d2d63] w-[15em] h-[20em]"}>

    </div>
}
