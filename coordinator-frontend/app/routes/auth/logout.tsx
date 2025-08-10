import {
    redirect,
    type LoaderFunctionArgs,
} from "react-router";
import {destroySession, getSession} from "~/lib/session";


export async function loader({ request }: LoaderFunctionArgs) {
    let session = await getSession(request);
    return redirect("/login", {
        headers: { "Set-Cookie": await destroySession(session) },
    });
}