import {type RouteConfig, index, route, layout} from "@react-router/dev/routes";

export default [

  layout("routes/site_layout.tsx", [
    index("routes/home.tsx"),
    route("register", "routes/auth/register.tsx"),
    route("login", "routes/auth/login.tsx"),
    route("dashboard", "routes/dashboard.tsx"),
    route("logout", "routes/auth/logout.tsx")
  ]),

  route("passkey/auth/start", "routes/passkey_auth_start.tsx"),
  route("passkey/register/start", "routes/passkey_register_start.tsx"),
  route("sign/start", "routes/device_sign_start.tsx"),
  route("sign/finish", "routes/device_sign_finish.tsx"),

] satisfies RouteConfig;
