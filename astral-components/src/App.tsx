import './App.css'
import {AlbumCard} from "./components/AlbumCard.tsx";
import {Player} from "./components/player/Player.tsx";
import {usePlaylistController} from "./util/PlaylistController.tsx";
import Track1 from "../audio/PrimoVere.flac";
import Track2 from "../audio/BubblegumCrisis.mp3";
import Track3 from "../audio/TheLament.mp3";
import Track4 from "../audio/PyrettaBlaze.flac";
import Track5 from "../audio/MentalCentralDialog.mp3";
import Track6 from "../audio/SatansHunger.flac";
import Track7 from "../audio/SecondHandEmbarrassment.mp3";

function App() {
    const { append, next, toggle } = usePlaylistController();

    return (
        <div className={"app-container"}>
            <div className="top-0 bottom-0 left-0 right-0 overflow-auto fixed bg-neutral-900">
                <div className="p-5 flex flex-row gap-5">
                    <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/5390a5a5cbef2a585da49609fd511d70.jpg"} name={"Down Below"} artist={"Tribulation"} description={"wawawawaw this is some album description read me please"} />
                    <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/976bf708fe0c03cfd1c17adf8f670d28.jpg"} name={"THE GHOST~POP TAPE"} artist={"Devon Hendryx"} description={"wawawawaw this is some album description read me please"} />
                    <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/937c42239633c3106f3299edd7c20da6.jpg"} name={"Murmuüre"} artist={"Murmuüre"} description={"wawawawaw this is some album description read me please"} />
                    <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/be18330817e87c2230f40bec80632aa5.jpg"} name={"Selected Ambient Works 85-92"} artist={"Aphex Twin"} description={"wawawawaw this is some album description read me please"} />
                    <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/5f5f967600ac3bca6e7007ae6c368dfa.jpg"} name={"Wallsocket"} artist={"underscores"} description={"wawawawaw this is some album description read me please"} />
                    <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/d2e8cc6713ee2fd861936bc0fb81deab.jpg"} name={"Konkurs"} artist={"Lifelover"} description={"wawawawaw this is some album description read me please"} />
                    <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/bb830427ca254d869290b316116fd372.jpg"} name={"October Rust"} artist={"Type O Negative"} description={"wawawawaw this is some album description read me please"} />
                </div>
            </div>

            <button className={"bg-red-500 absolute"} onClick={() => {
                append({
                    album: "Murmuüre", artist: "Murmuüre",
                    coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/937c42239633c3106f3299edd7c20da6.jpg",
                    format: "flac",
                    streamUrl: Track1,
                    title: "Primo Vere"
                })
                append({
                    album: "THE GHOST~POP TAPE", artist: "Devon Hendryx",
                    coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/976bf708fe0c03cfd1c17adf8f670d28.jpg",
                    format: "mp3",
                    streamUrl: Track2,
                    title: "Bubblegum Crisis"
                })
                append({
                    album: "Down Below", artist: "Tribulation",
                    coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/5390a5a5cbef2a585da49609fd511d70.jpg",
                    format: "mp3",
                    streamUrl: Track3,
                    title: "The Lament"
                })
                append({
                    album: "World Coming Down", artist: "Type O Negative",
                    coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/c31e9911d92c44a7b6312ceb156bf78d.jpg",
                    format: "flac",
                    streamUrl: Track4,
                    title: "Pyretta Blaze"
                })
                append({
                    album: "Konkurs", artist: "Lifelover",
                    coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/d2e8cc6713ee2fd861936bc0fb81deab.jpg",
                    format: "mp3",
                    streamUrl: Track5,
                    title: "Mental Central Dialog"
                })
                append({
                    album: "Sworn to the Dark", artist: "Watain",
                    coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/333ee44a25a205514d4b4ccfa9e57f2b.jpg",
                    format: "flac",
                    streamUrl: Track6,
                    title: "Satan's Hunger"
                })
                append({
                    album: "fishmonger", artist: "underscores",
                    coverUrl: "https://lastfm.freetls.fastly.net/i/u/770x0/0074590f78c850626134e0c01b3af7d1.jpg",
                    format: "mp3",
                    streamUrl: Track7,
                    title: "Second hand embarrassment"
                })

                next()
                toggle()

            }}>Add tracks to queue</button>

            <Player />

        </div>
    )
}

export default App
