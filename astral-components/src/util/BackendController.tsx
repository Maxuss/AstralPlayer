import {useCallback, useEffect, useState} from "react";
import axios, {AxiosError} from "axios";
import {singletonHook} from "react-singleton-hook";

export interface BackendController {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    get: (url: string) => Promise<any>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    post: (url: string, body: unknown, mtd: 'POST' | 'PATCH' | undefined) => Promise<any>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    postFile: (url: string, form: unknown) => Promise<any>,
    login: (username: string, password: string) => Promise<void>,
    setUrl: (newUrl: string) => void,
    loading: boolean
}

export function extractCookie(name: string): string | undefined {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    return parts.pop()?.split(';').shift()
}

export function setCookie(name: string, value: string, expirationHours: number) {
    const date = new Date();
    date.setTime(date.getTime() + (expirationHours * 60 * 60 * 1000));
    const expires = "; expires=" + date.toDateString();
    document.cookie = name + "=" + value + expires + "; path=/";
}
const useInitializeBackendController: () => BackendController = () => {
    const [refreshKey, setRefreshKey] = useState<string | undefined>(extractCookie("refresh-token"))
    const [accessKey, setAccessKey] = useState<string | undefined>(extractCookie("auth-token"))
    const [lastTokenValidation, setLastTokenValidation] = useState(-1)
    const [url, setUrl] = useState(extractCookie("backend-url"));

    useEffect(() => {
        setRefreshKey(extractCookie("refresh-token"))
        setAccessKey(extractCookie("auth-token"))

        if(refreshKey !== undefined && accessKey === undefined)
            requestAccessKey(refreshKey).then(() => { console.log("Fetched access key!")})
    }, []);

    useEffect(() => {
        // setting the refresh key cookie
        setCookie("refresh-token", refreshKey || "", 3 * 24);
    }, [refreshKey]);

    useEffect(() => {
        // setting the auth key cookie
        setCookie("auth-token", accessKey || "", 3 * 24);
    }, [accessKey])

    const requestAccessKey = useCallback(async (refresh: string) => {
        await axios({
            url: `${url}/auth/token`,
            method: 'GET',
            headers: {
                Authorization: `Bearer ${refresh}`
            }
        }).then(response => {
            const token = response.data as string
            setAccessKey(token)
        }).catch(error => {
            console.error(`Failed to obtain access key with error ${error.response.status} ("${error.response.data.error_type}": ${error.response.data.message})`);
        })
    }, [url]);

    const validateAccessToken = useCallback(async () => {
        axios({
            url: `${url}/auth/verify`,
            method: 'POST',
            data: accessKey,
            headers: {
                "Content-Type": "text/plain"
            }
        }).then(async result => {
            if(result.data === false) {
                // need to refresh key
                console.log("REFRESHING")
                await requestAccessKey(refreshKey || "NONE").then(() => {
                    console.log("REFRESHED")
                })
            }
        }).catch(err => {
            console.error(`Unknown error occurred: ${(err as AxiosError).message}`)
        })
    }, [accessKey, refreshKey])

    const obtainAccessToken = useCallback(async () => {
        if(Date.now() - lastTokenValidation >= 60 * 1000) {
            // only validate token every second
            await validateAccessToken();
            setLastTokenValidation(Date.now())
        }
        return accessKey
    }, [accessKey, lastTokenValidation, validateAccessToken])

    const login = async (username: string, password: string) => {
        await axios({
            url: `${url}/auth/login`,
            method: 'POST',
            data: {
                username: username,
                password: password,
            },
            headers: {
                "Content-Type": "application/json"
            }
        }).then(async response => {
            const token = response.data as string
            setRefreshKey(token)
            await requestAccessKey(token)
        }).catch(error => {
            if(error.response !== undefined)
                console.error(`Failed to login with error ${error.response.status} ("${error.response.data.error_type}": ${error.response.data.message})`);
        })
    }

    const get = useCallback(async (newUrl: string) => {
        return await axios({
            url: `${url}${newUrl}`,
            method: 'GET',
            headers: {
                Authorization: `Bearer ${await obtainAccessToken()}`
            }
        })
            .then(body => body.data)
            .catch(async error => {
                if(error.response !== undefined)
                    console.error(`Failed to perform a GET request to ${newUrl} -> ${error.response.status} ("${error.response.data.error_type}": ${error.response.data.message})`);
            })

    }, [url, obtainAccessToken])

    return {
        get: get,
        post: async (path, body, mtd) => {
            return await axios({
                url: `${url}${path}`,
                method: mtd || 'POST',
                headers: {
                    Authorization: `Bearer ${await obtainAccessToken()}`,
                    "Content-Type": "application/json"
                },
                data: body
            })
                .then(body => body.data)
                .catch(error => {
                    if(error.response !== undefined)
                        console.error(`Failed to perform a POST request to ${path} with body ${JSON.stringify(body)} -> ${error.response.status} ("${error.response.data.error_type}": ${error.response.data.message})`);
                })
        },
        postFile: async (path, body) => {
            return await axios.post(`${url}${path}`, body, {
                headers: {
                    Authorization: `Bearer ${await obtainAccessToken()}`,
                    'Content-Type': 'application/octet-stream'
                },
                maxBodyLength: Infinity,
                maxContentLength: Infinity,
            }).then(body => body.data)
                .catch(error => {
                    if(error.response !== undefined)
                        console.error(`Failed to perform a POST request to ${path} with body ${JSON.stringify(body)} -> ${error.response.status} ("${error.response.data.error_type}": ${error.response.data.message})`);
                })
        },
        login: login,
        loading: false,
        setUrl: setUrl
    } as BackendController
}

export const useBackendController = singletonHook<BackendController>({
    get: async () => { return { } },
    post: async () => { return { } },
    login: async () => { },
    loading: true,
    postFile: async () => { },
    setUrl: () => { }
    }, useInitializeBackendController
)