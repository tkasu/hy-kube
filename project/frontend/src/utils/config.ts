export const API_URL = process.env.REACT_APP_API_URL;
if (!API_URL) {
    throw Error("No `API_URL` defined");
}
