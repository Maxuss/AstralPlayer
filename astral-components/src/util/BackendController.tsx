import {useCallback, useEffect, useState} from "react";
import axios, {AxiosError} from "axios";
import {singletonHook} from "react-singleton-hook";

export interface BackendController {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    get: (url: string) => Promise<any>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    post: (url: string, body: unknown) => Promise<any>,
    login: (username: string, password: string) => Promise<void>,
    loading: boolean
}

function extractCookie(name: string): string | undefined {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    return parts.pop()?.split(';').shift()
}

function setCookie(name: string, value: string) {
    const date = new Date();
    date.setTime(date.getTime() + (3 * 24 * 60 * 60 *1000));
    const expires = "; expires=" + date.toDateString();
    document.cookie = name + "=" + value + expires + "; path=/";
}
const useInitializeBackendController: (baseUrl: string) => BackendController = (baseUrl) => {
    const [refreshKey, setRefreshKey] = useState<string | undefined>()
    const [accessKey, setAccessKey] = useState<string | undefined>()
    const [lastTokenValidation, setLastTokenValidation] = useState(-1)

    useEffect(() => {
        setRefreshKey(extractCookie("refresh-token"))
        setAccessKey(extractCookie("auth-token"))

        if(refreshKey !== undefined && accessKey === undefined)
            requestAccessKey(refreshKey).then(() => { console.log("Fetched access key!")})
    }, []);

    useEffect(() => {
        // setting the refresh key cookie
        setCookie("refresh-token", refreshKey || "");
    }, [refreshKey]);

    useEffect(() => {
        // setting the auth key cookie
        setCookie("auth-token", accessKey || "");
    }, [accessKey])

    const requestAccessKey = useCallback(async (refresh: string) => {
        await axios({
            url: `${baseUrl}/auth/token`,
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
    }, [baseUrl]);

    const validateAccessToken = useCallback(async () => {
        axios({
            url: `${baseUrl}/auth/verify`,
            method: 'POST',
            data: accessKey,
            headers: {
                "Content-Type": "text/plain"
            }
        }).then(async result => {
            if(result.data === "false") {
                // need to refresh key
                await requestAccessKey(refreshKey || "NONE")
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
            url: `${baseUrl}/auth/login`,
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

    const get = useCallback(async (url: string) => {
        return await axios({
            url: `${baseUrl}${url}`,
            method: 'GET',
            headers: {
                Authorization: `Bearer ${await obtainAccessToken()}`
            }
        })
            .then(body => body.data)
            .catch(async error => {
                if(error.response !== undefined)
                    console.error(`Failed to perform a GET request to ${url} -> ${error.response.status} ("${error.response.data.error_type}": ${error.response.data.message})`);
            })

    }, [baseUrl, obtainAccessToken])

    return {
        get: get,
        post: async (url, body) => {
            return await axios({
                url: `${baseUrl}${url}`,
                method: 'POST',
                headers: {
                    Authorization: `Bearer ${await obtainAccessToken()}`,
                    "Content-Type": "application/json"
                },
                data: body
            })
                .then(body => body.data)
                .catch(error => {
                    if(error.response !== undefined)
                        console.error(`Failed to perform a POST request to ${url} with body ${JSON.stringify(body)} -> ${error.response.status} ("${error.response.data.error_type}": ${error.response.data.message})`);
                })
        },
        login: login,
        loading: false,
    } as BackendController
}

export const useBackendController = singletonHook<BackendController>({
    get: async () => { return { } },
    post: async () => { return { } },
    login: async () => { },
    loading: true
    },
    () => useInitializeBackendController("http://localhost:8080") // TODO: custom URLs
)