use gami_sdk::GameInstallStatus;

#[derive(Debug, Clone, Copy)]
pub enum GameAction {
    Play,
    Install,
    Uninstall,
    Delete,
    Edit,
}
#[derive(Debug, Clone, Copy)]
pub struct GameActionData {
    pub name: &'static str,
    pub icon: &'static [u8],
    pub kind: GameAction,
}
const PLAY_ACTION: GameActionData = GameActionData {
    name: "Play",
    icon: include_bytes!("../../desktop/src/icons/tabler--play.svg"),
    kind: GameAction::Play,
};
const INSTALL_ACTION: GameActionData = GameActionData {
    name: "Install",
    icon: include_bytes!("../../desktop/src/icons/tabler--plus.svg"),
    kind: GameAction::Install,
};
const UNINSTALL_ACTION: GameActionData = GameActionData {
    name: "Uninstall",
    icon: include_bytes!("../../desktop/src/icons/tabler--minus.svg"),
    kind: GameAction::Uninstall,
};
const DELETE_ACTION: GameActionData = GameActionData {
    name: "Delete",
    icon: include_bytes!("../../desktop/src/icons/tabler--x.svg"),
    kind: GameAction::Delete,
};
const EDIT_ACTION: GameActionData = GameActionData {
    name: "Edit",
    icon: include_bytes!("../../desktop/src/icons/tabler--edit.svg"),
    kind: GameAction::Edit,
};
pub const fn get_actions(status: GameInstallStatus) -> &'static [GameActionData] {
    match status {
        GameInstallStatus::Installed => {
            &[PLAY_ACTION, UNINSTALL_ACTION, EDIT_ACTION, DELETE_ACTION]
        }
        _ => &[INSTALL_ACTION, EDIT_ACTION, DELETE_ACTION],
    }
}
