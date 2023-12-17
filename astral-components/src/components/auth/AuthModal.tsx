import './AuthModal.css';
import React, {useState} from "react";
import {setCookie, useBackendController} from "../../util/BackendController.tsx";
import {useNavigate} from "react-router-dom";

export interface AuthModalProps {
    then: string
}

export const AuthModal: React.FC<AuthModalProps> = ({ then }) => {
    // TODO: FINISH THIS!!!
    const [url, setUrlInner] = useState("");
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const navigate = useNavigate();
    const { login, setUrl } = useBackendController();

    return <div className={
        "md:mx-[25%] md:w-[50%] lg:mt-[5%] " + // desktop
        "w-[80%] mx-[10%] mt-9 h-[80%] " + // mobile
        "bg-zinc-950 outline-[#413a59] outline outline-2 rounded-xl " +
        "flex lg:flex-row flex-col"
    }>
        <div className={
            "lg:w-1/3 lg:h-full lg:rounded-l-xl lg:rounded-t-none " + // desktop
            "w-full h-1/3 rounded-t-xl " + // mobile
            "bg-zinc-900 flex flex-col"}>
            <h1 className={"font-montserrat text-violet-50 text-4xl ml-6 mt-4"}>
                Login
            </h1>
            {/*<img src="https://http.cat/404" alt={"Astral Logo"} className={*/}
            {/*    "md:mt-[50%] md:h-fit " + // desktop*/}
            {/*    "rounded-full w-[50%] my-5 ml-[25%]" // mobile*/}
            {/*} />*/}
        </div>
        <div className={"w-[100%] m-5 flex flex-col justify-left gap-5"}>
            {/* TODO: REPLACE ALL OF THIS IT LOOKS HORRIBLE */}
            <div>
                <h3 className={"text-xl text-zinc-50 pb-2"}>
                    Astral Server URL
                </h3>
                <input type={"text"} placeholder={"https://example.com"} className={
                    "w-[40%] rounded-lg bg-zinc-800 pl-2 focus:outline-[#caace0] outline focus:outline-2 outline-0 placeholder-zinc-400 text-zinc-50"
                } required={true} onInput={e => setUrlInner(e.target.value)}/>
            </div>
            <div>
                <h3 className={"text-xl text-zinc-50 pb-2"}>
                    Username
                </h3>
                <input type={"text"} placeholder={"Your username..."} className={
                    "w-[40%] rounded-lg bg-zinc-800 pl-2 focus:outline-[#caace0] outline focus:outline-2 outline-0 placeholder-zinc-400 text-zinc-50"
                } required={true} onInput={e => setUsername(e.target.value)}/>
            </div>
            <div>
                <h3 className={"text-xl text-zinc-50 pb-2"}>
                    Password
                </h3>
                <input type={"password"} placeholder={"Your password..."} className={
                    "w-[40%] rounded-lg bg-zinc-800 pl-2 focus:outline-[#caace0] outline focus:outline-2 outline-0 placeholder-zinc-400 text-zinc-50"
                } required={true} onInput={e => setPassword(e.target.value)} />
            </div>
            <div>
                <button
                    className={"text-lg text-zinc-zinc-950 bg-gradient-to-tr from-[#585170] to-[#a097bf] px-5 transition-all ease-in-out hover:text-zinc-200 hover:shadow-[#5a546d] shadow-md rounded-full"
                } onClick={() => {
                    if(url.length === 0 || username.length === 0 || password.length === 0)
                        return;

                    // TODO: regex? validation? nah too lazy
                    setUrl(url)
                    login(username, password).then(() => {
                        setCookie("backend-url", url, 1200 * 24);
                        navigate(then)
                    }).catch(err => {
                        // TODO: error handling
                        console.error("FAILED TO LOGIN")
                        console.error(err)
                    })
                }}>
                    Login
                </button>
            </div>
        </div>
    </div>
}