use crate::cli::config::Config;
use crate::cli::errors::{CliError, CliResult};
use crate::cli::output;
use crate::db::operations::{delete_peer, get_all_peers, initialize_database};

pub async fn list(json: bool, config: &Config) -> CliResult<()> {
    let db_path = config.db_path();
    let conn = initialize_database(&db_path).map_err(|e| CliError::DatabaseError(e.to_string()))?;

    let peers = get_all_peers(&conn).map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if json {
        let peers_json: Vec<_> = peers
            .iter()
            .map(|p| {
                serde_json::json!({
                    "peer_id": p.peer_id.to_string(),
                    "user_id": p.user_id.to_string(),
                    "device_id": p.device_id.to_string(),
                    "last_known_ip": p.last_known_ip,
                    "last_sync_time": p.last_sync_time,
                })
            })
            .collect();
        output::json(&serde_json::json!(peers_json));
    } else {
        if peers.is_empty() {
            output::info("No peers connected");
            return Ok(());
        }

        let mut table = output::create_table(vec!["Peer ID", "Device ID", "Last IP", "Last Sync"]);

        for peer in peers {
            table.add_row(prettytable::Row::new(vec![
                prettytable::Cell::new(&peer.peer_id.to_string()),
                prettytable::Cell::new(&peer.device_id.to_string()),
                prettytable::Cell::new(peer.last_known_ip.as_deref().unwrap_or("N/A")),
                prettytable::Cell::new(
                    &peer
                        .last_sync_time
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "Never".to_string()),
                ),
            ]));
        }

        table.printstd();
    }

    Ok(())
}

pub async fn add(multiaddr: &str, config: &Config) -> CliResult<()> {
    output::step(&format!("Adding peer: {}", multiaddr));

    output::warning("Direct peer addition requires IPC communication with the running daemon");
    output::info("Alternative approaches:");
    output::info("  1. Add bootstrap nodes to config file");
    output::info("  2. Use mDNS for automatic local peer discovery");
    output::info("  3. Configure relay servers in network settings");
    output::info("");
    output::info(&format!("To add bootstrap peer: {}", multiaddr));
    output::info("  Edit config file: ~/.ahenk/config.toml");
    output::info("  Add under [network.bootstrap_nodes]");

    // Future implementation will use IPC to communicate with daemon:
    // - Send ADD_PEER message to daemon via Unix socket/Named pipe
    // - Daemon will connect to peer and add to peer table
    // - Return success/failure status

    Ok(())
}

pub async fn remove(peer_id: &str, config: &Config) -> CliResult<()> {
    output::step(&format!("Removing peer: {}", peer_id));

    let peer_uuid = uuid::Uuid::parse_str(peer_id)
        .map_err(|_| CliError::ValidationError("Invalid peer ID format".to_string()))?;

    let db_path = config.db_path();
    let conn = initialize_database(&db_path).map_err(|e| CliError::DatabaseError(e.to_string()))?;

    // Remove the peer from database
    let rows_affected =
        delete_peer(&conn, peer_uuid).map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if rows_affected > 0 {
        output::success(&format!("Peer {} removed from database", peer_id));
        output::warning("Note: Daemon restart may be required for changes to take effect");
    } else {
        output::warning(&format!("Peer {} not found", peer_id));
    }

    Ok(())
}

pub async fn info(peer_id: &str, json: bool, config: &Config) -> CliResult<()> {
    let db_path = config.db_path();
    let conn = initialize_database(&db_path).map_err(|e| CliError::DatabaseError(e.to_string()))?;

    let peer_uuid = uuid::Uuid::parse_str(peer_id)
        .map_err(|_| CliError::ValidationError("Invalid peer ID".to_string()))?;

    let peer = crate::db::operations::get_peer(&conn, peer_uuid)
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;

    if json {
        output::json(&serde_json::json!({
            "peer_id": peer.peer_id.to_string(),
            "user_id": peer.user_id.to_string(),
            "device_id": peer.device_id.to_string(),
            "last_known_ip": peer.last_known_ip,
            "last_sync_time": peer.last_sync_time,
        }));
    } else {
        output::print_box(
            "Peer Information",
            vec![
                ("Peer ID", &peer.peer_id.to_string()),
                ("User ID", &peer.user_id.to_string()),
                ("Device ID", &peer.device_id.to_string()),
                (
                    "Last Known IP",
                    peer.last_known_ip.as_deref().unwrap_or("N/A"),
                ),
                (
                    "Last Sync",
                    &peer
                        .last_sync_time
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "Never".to_string()),
                ),
            ],
        );
    }

    Ok(())
}
