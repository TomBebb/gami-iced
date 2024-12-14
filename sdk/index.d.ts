export interface GameLibraryRef {
    libraryType: string
    libraryId: string
    name: string
}

export interface FetchOptions {
    method: "GET" | "POST" | "PUT" | "DELETE" | "OPTIONS" | "PATCH"
    headers: Record<string, string>
    body: string
}

declare namespace http {
    export function fetchText(url: string, args?: FetchOptions): Promise<string>;
}

declare namespace utils {
    export function openUrl(url: string): void;
}

export declare const enum GameInstallStatus {
    Installed = "Installed",
    Installing = "Installing",
    InLibrary = "InLibrary",
    Queued = "Queued",
    Uninstalling = "Uninstalling"
}

export interface ScannedGameLibraryMetadata extends GameLibraryRef {
    name: string,

    lastPlayed?: Date
    installStatus: GameInstallStatus
    playtime: number
    iconUrl?: string,
}

export interface GameLibrary extends GameAddonBase {
    launch(game: GameLibraryRef): Promise<void>;

    scan(): Promise<ScannedGameLibraryMetadata[]>;

    install(game: GameLibraryRef): Promise<void>;

    uninstall(game: GameLibraryRef): Promise<void>;

    check_install_status(game: GameLibraryRef): Promise<GameInstallStatus>;
}

export type ConfigTypes = {
    number: number;
    text: string;
    bool: boolean;
}

export type ConfigType = keyof ConfigTypes;
export type ConfigDeclaration = Record<string, { type: ConfigType, name: string, hint?: string }>
export type ConfigValue<TConfig extends ConfigDeclaration> = Record<keyof TConfig, TConfig[keyof TConfig]>;

export function getConfig<TConfig extends ConfigDeclaration>(key: string, conf: TConfig): ConfigValue<TConfig>;

// TODO
export interface GameMetadataScanner extends GameAddonBase {
}

export interface GameAddonBase {
    name: string,
    id: string,
}


export function registerLibrary(addon: GameLibrary): void;