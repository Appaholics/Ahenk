/// Example: Device Authorization Handshake (QR Pairing)
///
/// This example demonstrates the complete device pairing workflow:
/// 1. Existing device generates a QR code
/// 2. New device scans the QR code
/// 3. Devices perform cryptographic handshake
/// 4. New device is authorized and added to the account
///
/// Run this example:
///   cargo run --example device_pairing
use ahenk::*;
use libp2p::identity;
use std::io::{self, Write};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║       Device Authorization Handshake Demo (QR Pairing)   ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Simulate an existing user with a device
    let user_id = uuid::Uuid::new_v4();
    let existing_device_id = uuid::Uuid::new_v4();

    println!("👤 User Account");
    println!("   User ID: {}", user_id);
    println!("   Existing Device ID: {}\n", existing_device_id);

    // ========================================================================
    // STEP 1: EXISTING DEVICE GENERATES QR CODE
    // ========================================================================
    println!("═══ Step 1: Existing Device Generates QR Code ═══\n");

    // Generate keypair for existing device
    let existing_keypair = identity::Keypair::generate_ed25519();
    let existing_peer_id = libp2p::PeerId::from(existing_keypair.public());

    println!("📱 Existing Device (Authorizer):");
    println!("   Peer ID: {}", existing_peer_id);
    println!("   Listening on: /ip4/192.168.1.100/tcp/4001");

    // Create authorization workflow
    let mut authorizer = AuthorizerWorkflow::new();

    // Generate QR code data
    let qr_data = authorizer.generate_qr_code(
        user_id,
        existing_device_id,
        &existing_keypair,
        "/ip4/192.168.1.100/tcp/4001".to_string(),
    )?;

    println!("\n✅ QR Code Generated!");
    println!("   Length: {} bytes", qr_data.len());
    println!("   Valid for: 5 minutes");

    // Display QR code content (in real app, this would be shown as a visual QR code)
    println!("\n📊 QR Code Content (JSON):");
    println!("   {}", &qr_data[..qr_data.len().min(100)]);
    println!("   ...");

    // ========================================================================
    // STEP 2: NEW DEVICE SCANS QR CODE
    // ========================================================================
    println!("\n\n═══ Step 2: New Device Scans QR Code ═══\n");

    // Simulate scanning the QR code
    let challenge = NewDeviceWorkflow::scan_qr_code(&qr_data)?;

    println!("📸 QR Code Scanned Successfully!");
    println!("   Challenge ID: {}", challenge.challenge_id);
    println!("   Authorizer: {}", challenge.authorizer_peer_id);
    println!("   Expires: {}", challenge.expires_at.format("%H:%M:%S"));

    // ========================================================================
    // STEP 3: NEW DEVICE CREATES PAIRING REQUEST
    // ========================================================================
    println!("\n\n═══ Step 3: New Device Creates Pairing Request ═══\n");

    // User provides device information
    print!("📝 Enter device type (phone/tablet/desktop/watch): ");
    io::stdout().flush()?;
    let mut device_type = String::new();
    io::stdin().read_line(&mut device_type)?;
    let device_type = device_type.trim().to_string();

    print!("📝 Enter device name (e.g., 'John's iPhone'): ");
    io::stdout().flush()?;
    let mut device_name = String::new();
    io::stdin().read_line(&mut device_name)?;
    let device_name = device_name.trim().to_string();

    // Generate keypair for new device
    let new_device_keypair = identity::Keypair::generate_ed25519();
    let new_peer_id = libp2p::PeerId::from(new_device_keypair.public());

    println!("\n📱 New Device:");
    println!("   Type: {}", device_type);
    println!("   Name: {}", device_name);
    println!("   Peer ID: {}", new_peer_id);

    // Create authorization response
    let auth_response = NewDeviceWorkflow::create_pairing_request(
        &challenge,
        device_type.clone(),
        device_name.clone(),
        &new_device_keypair,
    )?;

    println!("\n✅ Pairing Request Created!");
    println!("   Request ID: {}", auth_response.requesting_device_id);
    println!(
        "   Signed Nonce: {} bytes",
        auth_response.signed_nonce.len()
    );

    // ========================================================================
    // STEP 4: CONNECT TO AUTHORIZER
    // ========================================================================
    println!("\n\n═══ Step 4: Connect to Authorizer ═══\n");

    let connect_addr = NewDeviceWorkflow::connect_to_authorizer(&challenge)?;
    println!("🔗 Connecting to: {}", connect_addr);
    println!("   (In real app, this would establish P2P connection)");

    // ========================================================================
    // STEP 5: AUTHORIZER VALIDATES AND AUTHORIZES
    // ========================================================================
    println!("\n\n═══ Step 5: Authorizer Validates Request ═══\n");

    println!("🔐 Authorizer performing validation:");
    println!("   ✓ Checking challenge validity");
    println!("   ✓ Verifying challenge not expired");
    println!("   ✓ Verifying cryptographic signature");
    println!("   ✓ Checking if challenge already used");

    // Create in-memory database for demo
    let conn = initialize_database(":memory:")?;

    // Register user in database
    let user = register_user(
        &conn,
        "demo_user".to_string(),
        "demo@example.com".to_string(),
        "password123".to_string(),
    )?;

    // Update the user_id in our workflow (in real app, this would already match)
    let mut authorizer = AuthorizerWorkflow::new();
    let qr_data = authorizer.generate_qr_code(
        user.user_id,
        existing_device_id,
        &existing_keypair,
        "/ip4/192.168.1.100/tcp/4001".to_string(),
    )?;

    let challenge = NewDeviceWorkflow::scan_qr_code(&qr_data)?;
    let auth_response = NewDeviceWorkflow::create_pairing_request(
        &challenge,
        device_type.clone(),
        device_name.clone(),
        &new_device_keypair,
    )?;

    // Validate and authorize
    let result = authorizer.authorize_device(&conn, &auth_response, &new_device_keypair)?;

    // ========================================================================
    // STEP 6: DISPLAY RESULT
    // ========================================================================
    println!("\n\n═══ Authorization Result ═══\n");

    match result {
        AuthResult::Success {
            device_id,
            user_id,
            sync_key,
        } => {
            println!("✅ AUTHORIZATION SUCCESSFUL!\n");
            println!("   New Device Added to Account:");
            println!("   ├─ Device ID: {}", device_id);
            println!("   ├─ User ID: {}", user_id);
            println!("   ├─ Sync Key: {} bytes", sync_key.len());
            println!("   └─ Device Type: {}", device_type);

            // Verify device was added to database
            let device = get_device(&conn, device_id)?;
            if let Some(dev) = device {
                println!("\n   Database Verification:");
                println!("   ├─ Device ID: {}", dev.device_id);
                println!("   ├─ User ID: {}", dev.user_id);
                println!("   ├─ Type: {}", dev.device_type);
                println!(
                    "   └─ Last Seen: {}",
                    dev.last_seen
                        .map(|d| d.to_string())
                        .unwrap_or("N/A".to_string())
                );
            } else {
                println!("\n   Database Verification: Device not found");
            }

            println!("\n📱 The new device can now:");
            println!("   • Sync data with other devices");
            println!("   • Access user's synchronized database");
            println!("   • Participate in P2P network");
            println!("   • Authorize additional devices");
        }
        AuthResult::Failed { reason } => {
            println!("❌ AUTHORIZATION FAILED");
            println!("   Reason: {}", reason);
        }
        AuthResult::Expired => {
            println!("⏱️  CHALLENGE EXPIRED");
            println!("   Please scan a new QR code");
        }
        AuthResult::InvalidSignature => {
            println!("🔒 INVALID SIGNATURE");
            println!("   Cryptographic verification failed");
        }
    }

    // ========================================================================
    // STEP 7: SECURITY SUMMARY
    // ========================================================================
    println!("\n\n═══ Security Features ═══\n");
    println!("🔐 Security Measures Applied:");
    println!("   ✓ Ed25519 cryptographic signatures");
    println!("   ✓ Time-limited challenges (5 minutes)");
    println!("   ✓ One-time use challenges");
    println!("   ✓ Nonce-based challenge-response");
    println!("   ✓ Public key verification");
    println!("   ✓ Shared sync key generation");
    println!("   ✓ Peer ID authentication");

    println!("\n📊 Pairing Session Statistics:");
    println!("   Active Sessions: {}", authorizer.active_session_count());

    // Cleanup
    authorizer.cleanup();
    println!("   After Cleanup: {}", authorizer.active_session_count());

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║           Device Pairing Demo Complete! ✅               ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    Ok(())
}
