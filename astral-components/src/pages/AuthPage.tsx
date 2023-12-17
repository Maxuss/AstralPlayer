import {useNavigate} from "react-router-dom";
import {useEffect} from "react";
import {extractCookie} from "../util/BackendController.tsx";
import { AuthModal } from "../components/auth/AuthModal.tsx";

export default function AuthPage() {
    const navigate = useNavigate();
    const navigateTo = new URLSearchParams(window.location.search).get("then") || "/";
    useEffect(() => {
        const loggedIn = extractCookie("refresh-token")
        if(loggedIn !== "") {
            navigate(navigateTo)
        }
    }, [])

    return <>
        <AuthModal then={navigateTo} />
    </>
}