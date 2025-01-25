export interface GameLibraryRef {
    name: String;
    libraryType: String;
    libraryId: String;
}

export enum GameInstallStatus {
    Installed,
    Installing,
    InLibrary,
    Queued,
}

export interface ScannedGameLibraryMetadata extends GameLibraryRef {
    lastPlayed?: Date;
    installStatus: GameInstallStatus;
    playtimeSecs: number;
    iconUrl?: String;
}

export interface GenreData {
    name: String;
    libraryId: String;
}

export interface GameMetadata {
    description?: String;
    developers: String[];
    genres: GenreData[];
    platforms: String[];
    publishers: String[];
    series: String[];
    tags: String[];
    releaseDate: Date;
    iconUrl?: String;
    coverUrl?: String;
    headerUrl?: String;
}

export interface GameMetadataScanner {
    getMetadata(game: GameLibraryRef): Promise<GameMetadata | null>;

    getMetadatas(
        games: GameLibraryRef[],
    ): Promise<[GameLibraryRef, GameMetadata][]>;
}

export interface GameLibrary {
    scan(): Promise<ScannedGameLibraryMetadata[]>;

    launch(game: GameLibraryRef): void;

    install(game: GameLibraryRef): void;

    uninstall(game: GameLibraryRef): void;

    checkInstallStatus(game: GameLibraryRef): GameInstallStatus;
}

export interface PluginConfData<TKey extends string = string> {
    name: string;
    hint: string;
    key: TKey;
}

export type Plugin<TConf extends Record<string, any> = {}> = {
    name: string;
    id: String;
    confs?: PluginConfData<keyof TConf>[];
} & { library?: GameLibrary; metadata?: GameMetadataScanner };

export declare function registerPlugin(plugin: Plugin): void;
