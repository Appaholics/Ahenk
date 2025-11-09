/// Example of using the P2P sync system with bootstrap and relay
///
/// Run multiple instances:
/// Terminal 1 (Bootstrap node):
///   cargo run --example p2p_sync -- --port 4001 --name Alice
///
/// Terminal 2 (Connect to bootstrap):
///   cargo run --example p2p_sync -- --port 4002 --name Bob --bootstrap /ip4/127.0.0.1/tcp/4001/p2p/PEER_ID
///
/// Terminal 3 (Another peer):
///   cargo run --example p2p_sync -- --port 4003 --name Charlie --bootstrap /ip4/127.0.0.1/tcp/4001/p2p/PEER_ID
use nexus_core::*;
use std::env;
use std::sync::{Arc, Mutex};

#[async_std::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    let mut port = 4001u16;
    let mut name = "Peer".to_string();
    let mut bootstrap_nodes = Vec::new();
    let mut relay_servers = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" | "-p" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(4001);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--name" | "-n" => {
                if i + 1 < args.len() {
                    name = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--bootstrap" | "-b" => {
                if i + 1 < args.len() {
                    bootstrap_nodes.push(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--relay" | "-r" => {
                if i + 1 < args.len() {
                    relay_servers.push(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--help" | "-h" => {
                println!("Usage: p2p_sync [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -p, --port PORT          Listen port (default: 4001)");
                println!("  -n, --name NAME          Peer name (default: Peer)");
                println!("  -b, --bootstrap ADDR     Bootstrap node address");
                println!("  -r, --relay ADDR         Relay server address");
                println!("  -h, --help               Show this help");
                return Ok(());
            }
            _ => i += 1,
        }
    }

    println!("=== Nexus P2P Sync Example ===");
    println!("Name: {}", name);
    println!("Port: {}", port);

    // Initialize database
    let db_path = format!("nexus_{}.db", name.to_lowercase());
    println!("Database: {}", db_path);

    let conn = initialize_database(&db_path)?;

    // Register or login user
    let user = match register_user(
        &conn,
        name.clone(),
        format!("{}@example.com", name.to_lowercase()),
        "password123".to_string(),
    ) {
        Ok(u) => {
            println!("Registered new user: {}", u.user_name);
            u
        }
        Err(_) => {
            // User exists, login instead
            let u = login_user(&conn, &name, "password123")?;
            println!("Logged in as: {}", u.user_name);
            u
        }
    };

    // Add device
    let device_name = format!("{}'s Device", name);
    let device = add_device_to_user(&conn, user.user_id, device_name, None)?;
    println!("Device: {}", device.device_type);

    // Create Arc<Mutex<Connection>> for thread safety
    let conn = Arc::new(Mutex::new(conn));

    // Generate keypair and peer ID
    let (peer_id, keypair) = generate_device_id();
    println!("Peer ID: {}", peer_id);

    // Configure P2P
    let config = P2PConfig {
        enable_mdns: true,
        enable_relay: !relay_servers.is_empty(),
        bootstrap_nodes: bootstrap_nodes.clone(),
        relay_servers: relay_servers.clone(),
        heartbeat_interval: std::time::Duration::from_secs(10),
        max_message_size: 65536,
    };

    println!("\n=== Network Configuration ===");
    println!("mDNS: {}", config.enable_mdns);
    println!("Relay: {}", config.enable_relay);
    println!("Bootstrap nodes: {}", bootstrap_nodes.len());
    println!("Relay servers: {}", relay_servers.len());

    // Create sync manager
    let mut sync_manager = SyncManager::new(
        keypair,
        user.user_id,
        device.device_id,
        conn.clone(),
        config.clone(),
    )?;

    // Start listening
    println!("\n=== Starting Network ===");
    sync_manager.listen(port)?;
    println!("Listening on port {}", port);

    // Display connection info for other peers
    println!("\nTo connect other peers, use:");
    println!("  --bootstrap /ip4/127.0.0.1/tcp/{}/p2p/{}", port, peer_id);

    // Connect to bootstrap nodes if provided
    if !bootstrap_nodes.is_empty() || !relay_servers.is_empty() {
        println!("\nConnecting to network...");
        sync_manager.connect_to_network(&bootstrap_nodes, &relay_servers)?;
    }

    // Announce presence
    println!("Announcing presence...");
    sync_manager.announce_presence()?;

    // Request initial sync
    let since = chrono::Utc::now() - chrono::Duration::hours(24);
    println!("Requesting sync since: {}", since);
    sync_manager.request_sync(since)?;

    // Create some sample data for demonstration
    {
        let mut conn_guard = conn.lock().unwrap();

        // Create a task list
        match create_new_task_list(
            &mut *conn_guard,
            user.user_id,
            device.device_id,
            format!("{}'s Tasks", name),
        ) {
            Ok(list) => {
                println!("\nCreated task list: {}", list.name);

                // Add a sample task
                match add_task_to_list(
                    &mut *conn_guard,
                    user.user_id,
                    device.device_id,
                    list.list_id,
                    format!("Sample task from {}", name),
                ) {
                    Ok(task) => println!("Created task: {}", task.content),
                    Err(e) => eprintln!("Failed to create task: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to create task list: {}", e),
        }
    }

    println!("\n=== Running Event Loop ===");
    println!("Press Ctrl+C to stop\n");

    // Run event loop
    sync_manager.run().await?;

    Ok(())
}
