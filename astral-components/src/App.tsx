import './App.css'
import {AlbumCard} from "./components/AlbumCard.tsx";
import {Player} from "./components/Player.tsx";

function App() {

    return (
        <div className="top-0 bottom-0 left-0 right-0 overflow-auto fixed bg-neutral-900">
            <div className="p-5 flex flex-row gap-5">
                <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/5390a5a5cbef2a585da49609fd511d70.jpg"} name={"Down Below"} artist={"Tribulation"} description={"wawawawaw this is some album description read me please"} />
                <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/976bf708fe0c03cfd1c17adf8f670d28.jpg"} name={"THE GHOST~POP TAPE"} artist={"Devon Hendryx"} description={"wawawawaw this is some album description read me please"} />
                <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/937c42239633c3106f3299edd7c20da6.jpg"} name={"Murmuüre"} artist={"Murmuüre"} description={"wawawawaw this is some album description read me please"} />
                <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/be18330817e87c2230f40bec80632aa5.jpg"} name={"Selected Ambient Works 85-92"} artist={"Aphex Twin"} description={"wawawawaw this is some album description read me please"} />
                <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/5f5f967600ac3bca6e7007ae6c368dfa.jpg"} name={"Wallsocket"} artist={"underscores"} description={"wawawawaw this is some album description read me please"} />
                <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/d2e8cc6713ee2fd861936bc0fb81deab.jpg"} name={"Konkurs"} artist={"Lifelover"} description={"wawawawaw this is some album description read me please"} />
            </div>
            <div className="m-5">
                <Player />
            </div>
        </div>
    )
}

export default App
