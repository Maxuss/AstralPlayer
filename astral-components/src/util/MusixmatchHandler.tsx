export interface MusixmatchTrackData {
    name: string,
    album: string,
    artist: string,
    duration: number,
    coverUrl: string,
    releaseDate: string,
}

export async function musixmatchSearch(
    name: string | undefined,
    album: string | undefined,
    artist: string | undefined,
    spotifyId: string | undefined,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    get: (path: string) => Promise<any>,
): Promise<MusixmatchTrackData | undefined> {
    const uri = new URL("https://example.com/");
    uri.searchParams.append("q_album", album || "");
    uri.searchParams.append("q_artist", artist || "");
    uri.searchParams.append("q_artists", artist || "");
    uri.searchParams.append("q_track", name || "");
    uri.searchParams.append("track_spotify_id", spotifyId || "");
    uri.searchParams.append("q_duration", "");
    uri.searchParams.append("f_subtitle_length", "");
    uri.searchParams.append("usertoken", "2005218b74f939209bda92cb633c7380612e14cb7fe92dcd6a780f");

    return await get(uri.toString().replace("https://example.com/", "/metadata/musixmatch")).then(response => {
        const data = response.message.body.macro_calls;
        const status = data["matcher.track.get"].message.header.status_code
        if(status != 200) {
            switch (status) {
                case 404:
                    throw Error("Couldn't find data on Musixmatch!");
                case 401:
                    throw Error("Musixmatch timed out");
                default:
                    throw Error(`Musixmatch error: ${status}`)
            }
        }

        const meta = data["matcher.track.get"].message.body.track;
        const name = meta.track_name;
        const album_name = meta.album_name;
        const release_date = meta.first_release_date;
        const artist = meta.artist_name;
        const duration = meta.track_length;
        let coverUrl: string = "none";
        let foundCover = false;
        ["800x800", "500x500", "350x350", "100x100"].forEach(each => {
            if(foundCover)
                return;
            const cover = meta[`album_coverart_${each}`]
            if(cover !== "") {
                coverUrl = cover;
                foundCover = true;
            }
        });
        return {
            name: name,
            album: album_name,
            releaseDate: release_date,
            artist: artist,
            duration: duration,
            coverUrl: coverUrl
        }
    }).catch(err => {
        console.error("Failed to get musixmatch metadata");
        console.error(err);
        return undefined;
    })
}