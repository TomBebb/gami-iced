import {
    GameInstallStatus,
    GameLibraryRef,
    getConfig,
    registerLibrary,
    ScannedGameLibraryMetadata,
    utils
} from "@gami/sdk";
import {parse} from "@node-steam/vdf";
import {readdir, readFile} from "fs/promises";
import * as path from "node:path";
import openUrl = utils.openUrl;

interface AppState {
    appid: number;
    name: string;
    installdir: string;
    StateFlags: number;
    LastUpdated: number;
    LastPlayed: number;
    SizeOnDisk: number;
    StagingSize: number;
    buildid: number;

    UpdateResult: number;
    BytesToDownload: number;
    BytesDownloaded: number;
    BytesToStage: number;
    BytesStaged: number;
    TargetBuildID: number;
    UserConfig?: {
        language: string;
    }
}

type GameKv = { AppState: AppState };

async function readSteamInfo(path: string): Promise<GameKv> {
    const content = await readFile(path, 'binary')
    return parse(content)
}

function runSteamCmd(cmd: string, ref: GameLibraryRef,): Promise<void> {
    openUrl(`steam://${cmd}/${ref.libraryId}`)
    return Promise.resolve();
}

const steamPath = "/home/tom/.steam/steam";
const appPath = path.join(steamPath, "steamapps")
const config = getConfig('steam', {
    apiKey: {
        type: "text",
        name: "API Key",
        hint: "Obtain from [Steam API Dev page](https://steamcommunity.com/dev/apikey)"
    },
    steamId: {
        type: "number",
        name: "Steam ID",
        hint: "In decimal format. Obtain from [an online tool](https://www.steamidfinder.com)"
    }
})
registerLibrary({
    id: "steam",
    name: "Steam",
    install: runSteamCmd.bind(null, "install"),
    uninstall: runSteamCmd.bind(null, "uninstall"),

    launch: runSteamCmd.bind(null, "rungameid"),
    async scan(): Promise<ScannedGameLibraryMetadata[]> {
        const items: ScannedGameLibraryMetadata[] = [];
        for (const scanPath of await readdir(appPath)) {
            if (!scanPath.endsWith(".acf")) continue;
            const {AppState: app} = await readSteamInfo(path.join(appPath, scanPath));
            items.push({
                iconUrl: `file://${steamPath.replace('\\', '/')}/appcache/librarycache/${app.appid}_icon.jpg`,
                playtime: 0,
                name: app.name,
                libraryId: app.appid.toString(),
                libraryType: "steam",
                installStatus: app.BytesDownloaded === app.BytesToDownload && app.BytesDownloaded > 0 ? "Installed" : "Installing"
            })
        }
        return items
    },
    check_install_status(game: GameLibraryRef): Promise<GameInstallStatus> {
        return Promise.resolve("InLibrary")
    }
})