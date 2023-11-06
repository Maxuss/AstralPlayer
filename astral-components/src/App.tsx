import { useState } from 'react'
import './App.css'
import {AlbumCard} from "./components/AlbumCard.tsx";

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="top-0 bottom-0 left-0 right-0 overflow-auto fixed bg-neutral-900">
        <div className="p-5 flex-row space-y-5">
            <AlbumCard artUrl={"https://lastfm.freetls.fastly.net/i/u/770x0/5390a5a5cbef2a585da49609fd511d70.jpg"} name={"Down Below"} artist={"Tribulation"} description={"wawawawaw this is some album description read me please"} />
        </div>
    </div>
  )
}

export default App
