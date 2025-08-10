import {createCookieSessionStorage } from "react-router";

// Session storage for all data including JWT tokens
const cookieSecret = process.env.COOKIE_SECRET_KEY || "fallback-secret-key";
const useHttp = process.env.USE_HTTP === "true";

const sessionStorage = createCookieSessionStorage({
  cookie: {
    name: "__session",
    httpOnly: !useHttp,
    maxAge: 60 * 60 * 24 * 30, // 30 days
    secrets: [cookieSecret],
    secure: !useHttp,
    sameSite: "lax",
  },
});

// Session functions
export async function getSession(request: Request) {
  return sessionStorage.getSession(request.headers.get("cookie"));
}

export async function commitSession(session: any) {
  return sessionStorage.commitSession(session);
}

export async function destroySession(session: any) {
  return sessionStorage.destroySession(session);
}

// JWT Token management from session
export async function getJwtToken(request: Request) {
  const session = await getSession(request);
  return session.get('jwt_token');
}
