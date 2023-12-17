import './App.css'
import {Player} from "./components/player/Player.tsx";
import {usePlaylistController} from "./util/PlaylistController.tsx";
import {LyricsSidebar} from "./components/lyrics/LyricsSidebar.tsx";
import {extractCookie, useBackendController} from "./util/BackendController.tsx";
import {MainView} from "./components/view/MainView.tsx";
import AuthPage from "./pages/AuthPage.tsx";
import {BrowserRouter, redirect, Route, Routes} from "react-router-dom";


function App() {
    usePlaylistController(); // need to precache the playlist and backend controller
    useBackendController();

    return (
        <BrowserRouter>
            <Routes>
                <Route path={"/"} element={
                    <div className={"app-container top-0 bottom-0 left-0 right-0 overflow-auto fixed bg-[#0f0f0f]"}>
                        <MainView/>
                        <Player/>
                        <LyricsSidebar/>
                    </div>
                }/>
                <Route path={"/auth"} element={
                    <div className={"app-container top-0 bottom-0 left-0 right-0 overflow-auto fixed bg-[#020202] auth-bg"}>
                        <AuthPage/>
                    </div>
                } />
            </Routes>
        </BrowserRouter>

    )
}

export default App
