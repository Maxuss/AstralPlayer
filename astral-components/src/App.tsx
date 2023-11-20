import './App.css'
import {AlbumCard} from "./components/AlbumCard.tsx";
import {Player} from "./components/player/Player.tsx";
import {usePlaylistController} from "./util/PlaylistController.tsx";
import {LyricsSidebar} from "./components/lyrics/LyricsSidebar.tsx";

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
                    album: "fishmonger", artist: "underscores",
                    format: "mp3",
                    title: "Second hand embarrassment",
                    id: "f40275e5-3e56-46da-aa7f-04d1e2037b3a"
                })
                append({
                    album: "All My Heroes Are Cornballs", artist: "JPEGMAFIA",
                    format: "mp3",
                    title: "Free the Frail",
                    id: "2c588e22-9d6a-4afd-811a-e0bc1b377a2d"
                })
                append({
                    album: "1000 gecs", artist: "100 gecs",
                    format: "flac",
                    title: "hand crushed by a mallet",
                    id: "b076c92d-a6dd-4daa-a17e-3fca97f361fd"
                })
                append({
                    album: "volcanic bird enemy and the voiced concern", artist: "Lil Ugly Mane",
                    format: "flac",
                    title: "vpn",
                    id: "de577ed8-f6b0-4a31-b047-cf36d12f6464"
                })
                append({
                    album: "Down Below", artist: "Tribulation",
                    format: "mp3",
                    title: "The Lament",
                    id: "2b07ba68-d8eb-4362-8fc7-591f956a222f"
                })
                append({
                    album: "Down Below", artist: "Tribulation",
                    format: "mp3",
                    title: "Nightbound",
                    id: "2c9de5a4-ed99-4098-94de-385f07fdcbaf"
                })
                append({
                    album: "Antagonist", artist: "Praise the Plague",
                    format: "mp3",
                    title: "Minatory Aeons",
                    id: "1ef6188c-f155-41d3-8418-36dd63feeadc"
                })
                append({
                    album: "Wallsocket", artist: "underscores",
                    format: "flac",
                    title: "Cops and robbers",
                    id: "0b27a743-73d3-4400-b1cc-9a88031827f4"
                })
                append({
                    album: "Sworn to the Dark", artist: "Watain",
                    format: "flac",
                    title: "Satan's Hunger",
                    id: "b33692b4-f49f-406c-920a-14de094d63c3"
                })
                append({
                    album: "Ritual Music for the True Clochard", artist: "Urfaust",
                    format: "mp3",
                    title: "Verächtung wird einen messertragenden Schatten",
                    id: "96be6eb0-f888-49b7-b7cb-4f91576bd438"
                })
                append({
                    album: "Dariacore 2: Enter Here, Hell to the Left", artist: "c0ncernn",
                    format: "flac",
                    title: "...during pride month?",
                    id: "21c30484-7d63-483d-9ad7-c6da293baf69"
                })
                append({
                    album: "...", artist: "King Gizzard & The Lizard Wizard",
                    format: "mp3",
                    title: "Motor Spirit",
                    id: "9cecd0d4-8a05-47fa-90ae-3c11c501bba2"
                })
                append({
                    album: "OIL OF EVERY PEARL'S UNINSIDES", artist: "SOPHIE",
                    format: "mp3",
                    title: "Faceshopping",
                    id: "c882b35d-79c8-4fe8-9d25-a2d83ecf549c"
                })

                append({
                    album: "effective. Power", artist: "MIMIDEATH",
                    format: "flac",
                    title: "Calcium",
                    id: "8bddae17-8c1d-4fb5-8e7a-3d10a8680772"
                })
                append({
                    album: "effective. Power", artist: "MIMIDEATH",
                    format: "flac",
                    title: "Fucked Up in the Crib Drinkin' Doctor Bob",
                    id: "0ea7244d-e435-4bf8-8ed9-45d58a884f8f"
                })
                append({
                    album: "FOAR EVERYWUN FRUM MIMI", artist: "MIMIDEATH",
                    format: "flac",
                    title: "In the Yudio going #DRazy",
                    id: "cfc43852-f03d-4853-a316-02a40cfa8f5f"
                })
                append({
                    album: "FOAR EVERYWUN FRUM MIMI", artist: "MIMIDEATH",
                    format: "flac",
                    title: "abusive",
                    id: "2db16d61-fb0f-412c-9baf-303413b4355f"
                })

                next()
                toggle()

            }}>Add tracks to queue</button>

            <Player />

            <LyricsSidebar />

        </div>
    )
}

export default App
