import type {LoaderFunctionArgs} from "react-router";
import {getSession} from "~/lib/session";

export async function isAuthenticated({request}: LoaderFunctionArgs): Promise<boolean> {
    const session = await getSession(request);
    return session.get("jwt_token");
}