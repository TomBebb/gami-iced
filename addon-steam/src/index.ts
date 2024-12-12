import type {GameLibraryRef} from "@gami/sdk";
import {parse} from "@node-steam/vdf";
import {readdir, readFile} from "fs/promises";
import * as path from "node:path";

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

async function readKv(path: string): Promise<GameKv> {

}

function runSteamCmd(cmd: string, ref: GameLibraryRef,): Promise<void> {
    // openUrl(`steam://${cmd}/${ref.libraryId}`)
    return Promise.resolve()
}

const appPath = "/home/tom/.steam/steam/steamapps"
for (const scanPath of await readdir(appPath)) {
    if (!scanPath.endsWith(".acf")) continue;
    const content = await readFile(path.join(appPath, scanPath), 'utf-8')
    const val = parse(content)

    console.info(val)
}
/*
export default registerAddon({
    type: "library",
    id: "steam",
    name: "Steam",
    install: runSteamCmd.bind(null, "install"),
    uninstall: runSteamCmd.bind(null, "uninstall"),

    launch: runSteamCmd.bind(null, "rungameid"),
    scan(): Promise<ScannedGameLibraryMetadata[]> {
        return Promise.resolve([])
    },
    check_install_status(game: GameLibraryRef): Promise<GameInstallStatus> {
        return Promise.resolve(GameInstallStatus.InLibrary) as Promise<GameInstallStatus>;
    }
})*/