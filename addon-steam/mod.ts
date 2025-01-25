import {registerPlugin} from "../sdk/mod";

interface Config {
    apiKey: string;
}

registerPlugin<Config>({
    id: "steam",
    name: "Steam",
    confs: [
        {name: "API Key", key: "apiKey", hint: "Enter API Key"},
    ],
});
