import {useEffect, useState} from "react";
import * as metadata from "music-metadata-browser";
import {Buffer} from "buffer";
import * as process from "process";
import {musixmatchSearch, MusixmatchTrackData} from "../../../util/MusixmatchHandler.tsx";

async function fetchFileData(file: File): Promise<MusixmatchTrackData | undefined> {
    return await metadata.parseReadableStream(file.stream(), { size: file.size, mimeType: file.name })
        .then(async parsed => {
            const meta = parsed.common;
            return await musixmatchSearch(meta.title, meta.album, meta.artist, undefined);
        }).catch(err => {
            console.error("Failed to extract data from file");
            console.error(err);
            return undefined;
        })
}

export const UploadView = () => {
    const [files, setFiles] = useState<FileList | undefined>()
    const [extractedFiles, setExtractedFiles] = useState<MusixmatchTrackData[]>([]);

    useEffect(() => {
        // this is cursed tbh
        window.Buffer = Buffer;
        window.process = process;
    }, []);

    useEffect(() => {
        if(files === undefined)
            return;
        for(let i = 0; i < files.length; i++) {
            const file = files.item(0)!;
            fetchFileData(file).then(data => {
                // TODO: some reports on if failed to find data?
                if(data !== undefined) {
                    const newExtractedFiles = extractedFiles;
                    console.log(data);
                    newExtractedFiles.push(data);
                    setExtractedFiles(newExtractedFiles);
                }
            })
        }
    }, [files])

    return <>
        <div className={"mt-[1%]"}>
            <h1 className={"ml-[1.6em] text-4xl text-zinc-50 font-montserrat "}>
                Upload music to the server
            </h1>
            <p className={"ml-[3.2em] text-lg text-zinc-100 mt-2"}>
                Drag and drop your files here or select them to upload them to the server
            </p>

            <input type="file" name="fileUpload" accept={"audio/flac, audio/mpeg, audio/mp4"} multiple={true} onChange={e => setFiles(e.target.files as FileList)} />
        </div>
    </>
}