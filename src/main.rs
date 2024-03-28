use clap::Parser;
use swayipc::{Connection, Fallible, Node};

/// program to toggle applications by `app_id` / `class` (for xwayland apps)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// app_id or window class for xwayland windows to toggle
    #[arg(required = true)]
    app_id: String,

    /// optional matching criteria mark
    #[arg(long, short)]
    mark: Option<String>,

    /// command to spawn the application
    #[arg(required = true)]
    command: String,

    /// whether or not the window should be floating
    #[arg(short, long, default_value="false")]
    floating: bool,
}

fn find<'a>(tree: &Node, pred: &'a dyn Fn(&Node) -> bool) -> Option<(i64, bool)> {
    if pred(tree) {
        return Some((tree.id, tree.visible.unwrap_or(false)));
    }
    for n in tree.nodes.iter().chain(tree.floating_nodes.iter()) {
        if let Some(r) = find(n, &pred) {
            return Some(r);
        }
    }
    return None;
}

fn main() -> Fallible<()> {
    let args = Args::parse();
    let mut connection = Connection::new()?;
    let wayland_pred = |node: &Node| {
        if let Some(app_id) = &node.app_id {
            app_id.as_str() == args.app_id.as_str()
        } else {
            false
        }
    };

    let xwayland_pred = |node: &Node| {
        if let Some(properties) = &node.window_properties {
            if let Some(class) = &properties.class {
                class.as_str() == args.app_id.as_str()
            } else {
                false
            }
        } else {
            false
        }
    };

    let id_wayland = find(&connection.get_tree()?, &wayland_pred);
    let id_xwayland = find(&connection.get_tree()?, &xwayland_pred);

    let res = match (id_wayland, id_xwayland) {
        (Some((w,v)), _) => Some((w, v)),
        (None, Some((x,v))) => Some((x, v)),
        _ => None
    };

    if let Some((con_id, visible)) = res {
        if visible {
            eprintln!("visible -> moving to scratchpad");
            let response = connection.run_command(format!("[con_id=\"{con_id}\"] move scratchpad"))?;
            for r in response { r?; }
        } else {
            eprintln!("not visible -> moving to current workspace");
            let cmd = if args.floating {
                "move window to workspace current, focus"
            } else {
                "move window to workspace current, focus, floating disable"
            };
            let response = connection.run_command(format!("[con_id=\"{con_id}\"] {cmd}"))?;
            for r in response { r?; }
        }
    } else {
        eprintln!("spawning new");
        let response = connection.run_command(format!("exec {}", args.command))?;
        for r in response { r?; }
    }
    Ok(())
}

