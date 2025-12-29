use hyprland::data::{Client, Workspace};
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::shared::{HyprDataActive, HyprDataActiveOptional};

fn main() -> hyprland::Result<()> {
    // Get the special workspace name from command line args, default to "magic"
    let args: Vec<String> = std::env::args().collect();
    let special_workspace = args.get(1).map(|s| s.as_str()).unwrap_or("magic");

    // Get the currently focused window
    let Some(client) = Client::get_active()? else {
        eprintln!("No active window found");
        return Ok(());
    };

    // Check if we're on any special workspace
    if client.workspace.name.starts_with("special:") {
        // Move to the currently active regular workspace
        // When on a special workspace, there's still a regular workspace active underneath
        let active_workspace = Workspace::get_active().map_err(|e| {
            eprintln!("Failed to get active workspace - this should not be possible!");
            e
        })?;

        // Make sure we're not trying to move to another special workspace
        if !active_workspace.name.starts_with("special:") {
            Dispatch::call(DispatchType::MoveToWorkspace(
                WorkspaceIdentifierWithSpecial::Id(active_workspace.id),
                None,
            ))?;
            println!("Moved window from {} to workspace {}", client.workspace.name, active_workspace.id);
        } else {
            // Fallback to workspace 1 if somehow we're on another special workspace
            Dispatch::call(DispatchType::MoveToWorkspace(
                WorkspaceIdentifierWithSpecial::Id(1),
                None,
            ))?;
            println!("Moved window from {} to workspace 1", client.workspace.name);
            println!("{:#?}", client.workspace);
        }
    } else {
        // Move to special workspace from any regular workspace
        Dispatch::call(DispatchType::MoveToWorkspace(
            WorkspaceIdentifierWithSpecial::Special(Some(special_workspace)),
            None,
        ))?;
        println!("Moved window from workspace {} to special:{}", client.workspace.id, special_workspace);
    }

    Ok(())
}
