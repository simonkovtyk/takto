#[derive(Debug)]
pub enum HyprlandEventName {
  Unknown,
  Workspace,
  WorkspaceV2,
  FocusedMon,
  FocusedMonV2,
  ActiveWindow,
  ActiveWindowV2,
  Fullscreen,
  MonitorRemoved,
  MonitorRemovedV2,
  MonitorAdded,
  MonitorAddedV2,
  CreateWorkspace,
  CreateWorkspaceV2,
  DestroyWorkspace,
  DestroyWorkspaceV2,
  MoveWorkspace,
  MoveWorkspaceV2,
  RenameWorkspace,
  ActiveSpecial,
  ActiveSpecialV2,
  ActiveLayout,
  OpenWindow,
  CloseWindow,
  MoveWindow,
  MoveWindowV2,
  OpenLayer,
  CloseLayer,
  Submap,
  ChangeFloatingMode,
  Urgent,
  Screencast,
  WindowTitle,
  WindowTitleV2,
  ToggleGroup,
  MoveIntoGroup,
  MoveOutOfGroup,
  IgnoreGroupLock,
  LockGroups,
  ConfigReloaded,
  Pin,
  Minimized,
  Bell,
  VDesk
}

impl From<&str> for HyprlandEventName {
  fn from(s: &str) -> HyprlandEventName {
    match s {
      "workspace" => HyprlandEventName::Workspace,
      "workspacev2" => HyprlandEventName::WorkspaceV2,
      "focusedmon" => HyprlandEventName::FocusedMon,
      "focusedmonv2" => HyprlandEventName::FocusedMonV2,
      "activewindow" => HyprlandEventName::ActiveWindow,
      "activewindowv2" => HyprlandEventName::ActiveWindowV2,
      "fullscreen" => HyprlandEventName::Fullscreen,
      "monitorremoved" => HyprlandEventName::MonitorRemoved,
      "monitorremovedv2" => HyprlandEventName::MonitorRemovedV2,
      "monitoradded" => HyprlandEventName::MonitorAdded,
      "monitoraddedv2" => HyprlandEventName::MonitorAddedV2,
      "createworkspace" => HyprlandEventName::CreateWorkspace,
      "createworkspacev2" => HyprlandEventName::CreateWorkspaceV2,
      "destroyworkspace" => HyprlandEventName::DestroyWorkspace,
      "destroyworkspacev2" => HyprlandEventName::DestroyWorkspaceV2,
      "moveworkspace" => HyprlandEventName::MoveWorkspace,
      "moveworkspacev2" => HyprlandEventName::MoveWorkspaceV2,
      "renameworkspace" => HyprlandEventName::RenameWorkspace,
      "activespecial" => HyprlandEventName::ActiveSpecial,
      "activespecialv2" => HyprlandEventName::ActiveSpecialV2,
      "activelayout" => HyprlandEventName::ActiveLayout,
      "openwindow" => HyprlandEventName::OpenWindow,
      "closewindow" => HyprlandEventName::CloseWindow,
      "movewindow" => HyprlandEventName::MoveWindow,
      "movewindowv2" => HyprlandEventName::MoveWindowV2,
      "openlayer" => HyprlandEventName::OpenLayer,
      "closelayer" => HyprlandEventName::CloseLayer,
      "submap" => HyprlandEventName::Submap,
      "changefloatingmode" => HyprlandEventName::ChangeFloatingMode,
      "urgent" => HyprlandEventName::Urgent,
      "screencast" => HyprlandEventName::Screencast,
      "windowtitle" => HyprlandEventName::WindowTitle,
      "windowtitlev2" => HyprlandEventName::WindowTitleV2,
      "togglegroup" => HyprlandEventName::ToggleGroup,
      "moveintogroup" => HyprlandEventName::MoveIntoGroup,
      "moveoutofgroup" => HyprlandEventName::MoveOutOfGroup,
      "ignoregrouplock" => HyprlandEventName::IgnoreGroupLock,
      "lockgroups" => HyprlandEventName::LockGroups,
      "configreloaded" => HyprlandEventName::ConfigReloaded,
      "pin" => HyprlandEventName::Pin,
      "minimized" => HyprlandEventName::Minimized,
      "bell" => HyprlandEventName::Bell,
      "vdesk" => HyprlandEventName::VDesk,
      _ => HyprlandEventName::Unknown
  }
}}

const HYPRLAND_EVENT_SEPARATOR: &str = ">>";

#[derive(Debug)]
pub struct HyprlandEvent {
  pub name: HyprlandEventName,
  pub data: Option<String>
}

impl From<&str> for HyprlandEvent {
  fn from(value: &str) -> Self {
    let split = value.split(HYPRLAND_EVENT_SEPARATOR);
    let mut name = HyprlandEventName::Unknown;
    let mut data = None;

    for (index, value) in split.enumerate() {
      if index > 1 {
        break;
      }

      if index == 0 {
        name = HyprlandEventName::from(value);
      }

      if index == 1 {
        data = Some(value.to_string());
      }
    }

    return HyprlandEvent {
      name,
      data
    }
  }
}
