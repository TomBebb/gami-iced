export const enum KvTokenKind {
    OpenObject,
    CloseObject,
    String
}

export class KvToken {
    public constructor(public readonly kind: KvTokenKind, public readonly text: string) {
    }

}

