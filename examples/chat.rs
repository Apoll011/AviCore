use async_trait::async_trait;
use avi_device::capability::{CapabilityBuilder, SensorCapability};
use avi_device::device::{AviDevice, AviDeviceConfig, AviDeviceType};
use avi_device::stream::{StreamContext, StreamHandler, StreamHandlerFactory};
use avi_p2p::{PeerId, StreamCloseReason, StreamId};
use serde_json::json;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::RwLock;

mod commands;
mod domain;
mod message_types;
mod session;

use commands::{Command, CommandRegistry};
use domain::{ChatMessage, MessageType};
use session::SessionManager;

// ============================================================================
// Stream Handlers - Enhanced with session awareness
// ============================================================================

struct ChatStreamHandler {
    peer_id: Option<PeerId>,
    session_manager: Arc<SessionManager>,
}

impl ChatStreamHandler {
    fn new(session_manager: Arc<SessionManager>) -> Self {
        Self {
            peer_id: None,
            session_manager,
        }
    }
}

#[async_trait]
impl StreamHandler for ChatStreamHandler {
    async fn on_accepted(&mut self, ctx: &StreamContext) {
        self.peer_id = Some(ctx.peer_id.clone());

        // Register session
        if let Some(session) = self
            .session_manager
            .create_session(ctx.stream_id, ctx.peer_id.to_string())
            .await
        {
            println!("\nChat stream established with {}", ctx.peer_id);
            println!("   Session ID: {}", session.read().await.id);
            println!("   Type 'msg <text>' to send messages");
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }

    async fn on_rejected(&mut self, peer_id: PeerId, _stream_id: StreamId, reason: String) {
        println!("\n Chat request to {} rejected: {}", peer_id, reason);
        print!("> ");
        io::stdout().flush().unwrap();
    }

    async fn on_data(&mut self, ctx: &StreamContext, data: Vec<u8>) {
        // Try to parse as ChatMessage first
        if let Ok(chat_msg) = serde_json::from_slice::<ChatMessage>(&data) {
            self.handle_chat_message(ctx, chat_msg).await;
        } else if let Ok(msg) = String::from_utf8(data.clone()) {
            // Fallback to plain text
            println!("\n💬 [{}] {}", ctx.peer_id, msg);

            // Update session
            if let Some(session) = self
                .session_manager
                .get_session_by_stream(ctx.stream_id)
                .await
            {
                session
                    .write()
                    .await
                    .add_message(ctx.peer_id.to_string(), msg, MessageType::Text);
            }
        } else {
            println!(
                "\n Received binary data from {} ({} bytes)",
                ctx.peer_id,
                data.len()
            );
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }

    async fn on_closed(&mut self, peer_id: PeerId, stream_id: StreamId, reason: StreamCloseReason) {
        println!("\n Chat with {} closed ({:?})", peer_id, reason);

        // Close session
        self.session_manager.close_session(stream_id).await;

        print!("> ");
        io::stdout().flush().unwrap();
    }
}

impl ChatStreamHandler {
    async fn handle_chat_message(&self, ctx: &StreamContext, msg: ChatMessage) {
        match msg.message_type {
            MessageType::Text => {
                println!("\n [{}] {}", ctx.peer_id, msg.content);
            }
            MessageType::Typing => {
                println!("\n  {} is typing...", ctx.peer_id);
            }
            MessageType::System => {
                println!("\n System from {}: {}", ctx.peer_id, msg.content);
            }
            MessageType::Command => {
                println!("\n  Command from {}: {}", ctx.peer_id, msg.content);
            }
        }

        // Update session
        if let Some(session) = self
            .session_manager
            .get_session_by_stream(ctx.stream_id)
            .await
        {
            session
                .write()
                .await
                .add_message(msg.sender, msg.content, msg.message_type);
        }
    }
}

struct ChatStreamFactory {
    session_manager: Arc<SessionManager>,
}

impl ChatStreamFactory {
    fn new(session_manager: Arc<SessionManager>) -> Self {
        Self { session_manager }
    }
}

#[async_trait]
impl StreamHandlerFactory for ChatStreamFactory {
    async fn create_handler(&self) -> Box<dyn StreamHandler> {
        Box::new(ChatStreamHandler::new(self.session_manager.clone()))
    }
}

// ============================================================================
// Application State
// ============================================================================

struct AppState {
    device: Arc<AviDevice>,
    session_manager: Arc<SessionManager>,
    command_registry: Arc<CommandRegistry>,
    active_stream: Arc<RwLock<Option<StreamId>>>,
    subscriptions: Arc<RwLock<HashMap<String, String>>>, // topic -> subscription_id
}

impl AppState {
    async fn new(device: Arc<AviDevice>) -> Self {
        let session_manager = Arc::new(SessionManager::new());
        let command_registry = Arc::new(CommandRegistry::new());

        Self {
            device,
            session_manager,
            command_registry,
            active_stream: Arc::new(RwLock::new(None)),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// ============================================================================
// Main Application
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), String> {
    print_banner();

    let device = initialize_device().await?;
    let state = Arc::new(AppState::new(device.clone()).await);

    // Register stream handler with session manager
    device
        .register_stream_handler(
            "chat".to_string(),
            ChatStreamFactory::new(state.session_manager.clone()),
        )
        .await;

    device.start_event_loop();

    // Setup startup handler
    let state_clone = state.clone();
    device
        .on_started(move |dev, peer_id, _listening| on_started(dev, peer_id, state_clone.clone()))
        .await;

    // Run CLI loop
    run_cli_loop(state).await?;

    Ok(())
}

fn print_banner() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                       AVI Chat CLI                         ║");
    println!("║                                                            ║");
    println!("║      Type 'help' for commands or 'tutorial' for a guide    ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
}

async fn initialize_device() -> Result<Arc<AviDevice>, String> {
    let caps = CapabilityBuilder::new()
        .sensor(
            "microphone",
            SensorCapability::Microphone {
                present: true,
                array_size: 2,
                sampling_rate_khz: 44,
                max_spl_db: 110,
            },
        )
        .sensor(
            "cli_node",
            SensorCapability::Temperature {
                present: true,
                accuracy_celsius: 0.1,
                current_value: Some(1.0),
            },
        )
        .build();

    let node_name = format!("cli-enhanced-{}", std::process::id());
    let config = AviDeviceConfig {
        node_name,
        device_type: AviDeviceType::NODE,
        capabilities: caps,
        can_gateway_embedded: true,
    };

    Ok(Arc::new(AviDevice::new(config).await?))
}

async fn run_cli_loop(state: Arc<AppState>) -> Result<(), String> {
    let mut lines = BufReader::new(tokio::io::stdin()).lines();

    print!("> ");
    io::stdout().flush().unwrap();

    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            print!("> ");
            io::stdout().flush().unwrap();
            continue;
        }

        // Parse and execute command
        match parse_command(&line) {
            Ok(cmd) => {
                if let Err(e) = execute_command(cmd, &state).await {
                    println!(" Error: {}", e);
                }
            }
            Err(e) => {
                println!(" Invalid command: {}", e);
                println!("   Type 'help' for available commands");
            }
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }

    Ok(())
}

fn parse_command(line: &str) -> Result<Command, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    Command::from_parts(&parts)
}

async fn execute_command(cmd: Command, state: &AppState) -> Result<(), String> {
    match cmd {
        Command::Help => show_help(),
        Command::Tutorial => show_tutorial(),
        Command::Exit => {
            println!("Goodbye!");
            std::process::exit(0);
        }

        // Network commands
        Command::Peers => list_peers(&state.device).await?,
        Command::Status => show_status(&state.device).await?,
        Command::Query => query_nodes(&state.device).await?,

        // PubSub commands
        Command::Subscribe { topic } => subscribe(state, topic).await?,
        Command::SubscribeTyped { topic, type_name } => {
            subscribe_typed(state, topic, type_name).await?
        }
        Command::Unsubscribe { topic } => unsubscribe(state, topic).await?,
        Command::Publish { topic, message } => publish(state, topic, message).await?,
        Command::PublishTyped { topic, json_data } => {
            publish_typed(state, topic, json_data).await?
        }
        Command::ListSubscriptions => list_subscriptions(state).await,

        // Context commands
        Command::SetContext { path, value } => set_context(&state.device, path, value).await?,
        Command::GetContext { path } => get_context(&state.device, path).await?,
        Command::SetSpeaker => set_speaker(&state.device).await?,

        // Intent commands
        Command::Intent { message } => send_intent(&state.device, message).await?,

        // Stream/Chat commands
        Command::Call { peer_id } => call_peer(state, peer_id).await?,
        Command::Message { text } => send_message(state, text).await?,
        Command::Typing => send_typing_indicator(state).await?,
        Command::Hangup => hangup(state).await?,
        Command::Sessions => list_sessions(state).await,
        Command::History { session_id } => show_history(state, session_id).await?,
        Command::SwitchSession { session_id } => switch_session(state, session_id).await?,

        // Advanced commands
        Command::Broadcast { message } => broadcast_message(state, message).await?,
        Command::Stats => show_stats(state).await,
        Command::Clear => {
            print!("\x1B[2J\x1B[1;1H");
            print_banner();
        }
    }

    Ok(())
}

// ============================================================================
// Command Implementations
// ============================================================================

fn show_help() {
    println!("\nAvailable Commands:\n");
    println!("  Network & Discovery:");
    println!("    peers                    - List connected peers");
    println!("    status                   - Show local node info");
    println!("    query                    - Find other CLI nodes");
    println!();
    println!("  PubSub:");
    println!("    sub <topic>              - Subscribe to topic");
    println!("    sub-typed <topic> <type> - Subscribe with type conversion (json, text, etc.)");
    println!("    unsub <topic>            - Unsubscribe from topic");
    println!("    pub <topic> <msg>        - Publish to topic");
    println!("    pub-typed <topic> <json> - Publish typed JSON message");
    println!("    subs                     - List active subscriptions");
    println!();
    println!("  Chat & Streams:");
    println!("    call <peer_id>           - Start chat with peer");
    println!("    msg <text>               - Send message to active chat");
    println!("    typing                   - Send typing indicator");
    println!("    hangup                   - Close active chat");
    println!("    sessions                 - List all chat sessions");
    println!("    history [session_id]     - Show chat history");
    println!("    switch <session_id>      - Switch to different session");
    println!();
    println!("  Context & State:");
    println!("    set <path> <value>       - Update context");
    println!("    get <path>               - Get context value");
    println!("    speaker                  - Set self as speaker");
    println!();
    println!("  Advanced:");
    println!("    intent <msg>             - Send intent for processing");
    println!("    broadcast <msg>          - Broadcast to all sessions");
    println!("    stats                    - Show statistics");
    println!("    clear                    - Clear screen");
    println!();
    println!("  Other:");
    println!("    tutorial                 - Show interactive tutorial");
    println!("    help                     - Show this help");
    println!("    exit                     - Quit application");
    println!();
}

fn show_tutorial() {
    println!("\nQuick Start Tutorial:\n");
    println!("Check your status:    status");
    println!("Find other nodes:     query");
    println!("Start a chat:         call <peer_id_from_query>");
    println!("Send a message:       msg Hello, world!");
    println!("Subscribe to topics:  sub global");
    println!("Publish messages:     pub global Hi everyone!");
    println!();
    println!("Pro Tips:");
    println!("  • Use 'sub-typed' for automatic JSON deserialization");
    println!("  • Multiple chat sessions are supported - use 'sessions' to see them");
    println!("  • 'typing' sends a typing indicator to your chat partner");
    println!("  • Check 'stats' to see your activity summary");
    println!();
}

async fn list_peers(device: &AviDevice) -> Result<(), String> {
    match device.get_peers().await {
        Ok(peers) => {
            if peers.is_empty() {
                println!(" No connected peers");
            } else {
                println!(" Connected Peers ({}):", peers.len());
                for peer in peers {
                    println!("   • {}", peer);
                }
            }
        }
        Err(e) => return Err(format!("Failed to get peers: {}", e)),
    }
    Ok(())
}

async fn show_status(device: &AviDevice) -> Result<(), String> {
    println!("\n Node Status:\n");

    match device.get_core_id().await {
        Ok(id) => println!("  Core ID: {}", id),
        Err(_) => println!("  Core ID:  Waiting for discovery..."),
    }

    println!("  Local ID: {}", device.get_id().await);

    if let Ok(ctx) = device.get_ctx("").await
        && let Some(obj) = ctx.as_object()
    {
        println!("\n  Context Keys ({}):", obj.keys().len());
        for key in obj.keys() {
            println!("    • {}", key);
        }
    }

    println!();
    Ok(())
}

async fn query_nodes(device: &AviDevice) -> Result<(), String> {
    use avi_device::DeviceQuery;

    println!("Searching for CLI nodes...");
    let query = DeviceQuery::new().sensor("cli_node", |_| true);

    match device.execute_query(query).await {
        Ok(results) => {
            if results.is_empty() {
                println!("   No CLI nodes found");
            } else {
                println!("   Found {} CLI node(s):", results.len());
                for peer_id in results {
                    println!("     • {}", peer_id);
                }
            }
        }
        Err(e) => return Err(format!("Query failed: {}", e)),
    }

    Ok(())
}

async fn subscribe(state: &AppState, topic: String) -> Result<(), String> {
    let _topic_clone = topic.clone();

    state
        .device
        .subscribe(&topic, move |from, topic, data| {
            let msg = String::from_utf8_lossy(&data);
            println!("\n [{}] {} → {}", topic, from, msg);
            print!("> ");
            let _ = io::stdout().flush();
        })
        .await
        .map_err(|e| format!("Failed to subscribe: {}", e))?;

    state
        .subscriptions
        .write()
        .await
        .insert(topic.clone(), topic.clone());
    println!(" Subscribed to '{}'", topic);

    Ok(())
}

async fn subscribe_typed(state: &AppState, topic: String, type_name: String) -> Result<(), String> {
    let _topic_clone = topic.clone();
    let type_clone = type_name.clone();

    state
        .device
        .subscribe(&topic, move |from, topic, data| {
            match type_clone.as_str() {
                "json" => {
                    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&data) {
                        println!(
                            "\n[{}] {} → {}",
                            topic,
                            from,
                            serde_json::to_string_pretty(&json).unwrap()
                        );
                    } else {
                        println!("\n  [{}] Failed to parse as JSON", topic);
                    }
                }
                "text" | "string" => {
                    let msg = String::from_utf8_lossy(&data);
                    println!("\n [{}] {} → {}", topic, from, msg);
                }
                "hex" => {
                    let hex = data
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    println!("\n [{}] {} → {}", topic, from, hex);
                }
                "chat" => {
                    if let Ok(chat_msg) = serde_json::from_slice::<ChatMessage>(&data) {
                        println!("\n [{}] {}: {}", topic, chat_msg.sender, chat_msg.content);
                    } else {
                        let msg = String::from_utf8_lossy(&data);
                        println!("\n [{}] {} → {}", topic, from, msg);
                    }
                }
                _ => {
                    let msg = String::from_utf8_lossy(&data);
                    println!("\n [{}] {} → {} (type: {})", topic, from, msg, type_clone);
                }
            }
            print!("> ");
            let _ = io::stdout().flush();
        })
        .await
        .map_err(|e| format!("Failed to subscribe: {}", e))?;

    state
        .subscriptions
        .write()
        .await
        .insert(topic.clone(), format!("{} (type: {})", topic, type_name));

    println!(" Subscribed to '{}' with type '{}'", topic, type_name);
    Ok(())
}

async fn unsubscribe(state: &AppState, topic: String) -> Result<(), String> {
    // Note: AviDevice doesn't expose unsubscribe, so we just remove from tracking
    if state.subscriptions.write().await.remove(&topic).is_some() {
        println!(" Unsubscribed from '{}'", topic);
    } else {
        println!("  Not subscribed to '{}'", topic);
    }
    Ok(())
}

async fn list_subscriptions(state: &AppState) {
    let subs = state.subscriptions.read().await;
    if subs.is_empty() {
        println!(" No active subscriptions");
    } else {
        println!("\n Active Subscriptions ({}):", subs.len());
        for (_topic, desc) in subs.iter() {
            println!("   • {}", desc);
        }
        println!();
    }
}

async fn publish(state: &AppState, topic: String, message: String) -> Result<(), String> {
    state
        .device
        .publish(&topic, message.into_bytes())
        .await
        .map_err(|e| format!("Failed to publish: {}", e))?;

    println!(" Published to '{}'", topic);
    Ok(())
}

async fn publish_typed(state: &AppState, topic: String, json_data: String) -> Result<(), String> {
    let value: serde_json::Value =
        serde_json::from_str(&json_data).map_err(|e| format!("Invalid JSON: {}", e))?;

    let bytes = serde_json::to_vec(&value).map_err(|e| format!("Failed to serialize: {}", e))?;

    state
        .device
        .publish(&topic, bytes)
        .await
        .map_err(|e| format!("Failed to publish: {}", e))?;

    println!(" Published JSON to '{}'", topic);
    Ok(())
}

async fn set_context(device: &AviDevice, path: String, value: String) -> Result<(), String> {
    let json_val = serde_json::from_str(&value).unwrap_or(json!(value));

    device
        .update_ctx(&path, json_val)
        .await
        .map_err(|e| format!("Failed to update context: {}", e))?;

    println!(" Context '{}' updated", path);
    Ok(())
}

async fn get_context(device: &AviDevice, path: String) -> Result<(), String> {
    let path_str = if path.is_empty() { "root" } else { &path };

    match device.get_ctx(&path).await {
        Ok(v) => {
            println!("\n Context '{}':", path_str);
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
            println!();
        }
        Err(e) => return Err(format!("Failed to get context: {}", e)),
    }

    Ok(())
}

async fn set_speaker(device: &AviDevice) -> Result<(), String> {
    let speaker_id = device.get_id().await.to_string();
    let json_val = json!(speaker_id);

    device
        .update_ctx("avi.dialogue.speaker", json_val)
        .await
        .map_err(|e| format!("Failed to set speaker: {}", e))?;

    println!(" Set as speaker: {}", speaker_id);
    Ok(())
}

async fn send_intent(device: &AviDevice, message: String) -> Result<(), String> {
    device
        .publish("intent/execute/text", message.into_bytes())
        .await
        .map_err(|e| format!("Failed to send intent: {}", e))?;

    println!(" Intent sent");
    Ok(())
}

async fn call_peer(state: &AppState, peer_id: String) -> Result<(), String> {
    let peer = PeerId::new(&peer_id);

    match state
        .device
        .request_stream(peer.clone(), "chat".to_string())
        .await
    {
        Ok(stream_id) => {
            println!(" Calling {}...", peer_id);
            println!("   Stream ID: {}", stream_id);

            // Set as active stream
            *state.active_stream.write().await = Some(stream_id);
        }
        Err(e) => return Err(format!("Failed to request stream: {}", e)),
    }

    Ok(())
}

async fn send_message(state: &AppState, text: String) -> Result<(), String> {
    let stream_id = *state.active_stream.read().await;

    if let Some(id) = stream_id {
        let sender_id = state.device.get_id().await.to_string();

        let msg = ChatMessage {
            sender: sender_id.clone(),
            content: text.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            message_type: MessageType::Text,
        };

        let data =
            serde_json::to_vec(&msg).map_err(|e| format!("Failed to serialize message: {}", e))?;

        state
            .device
            .send_stream_data(id, data)
            .await
            .map_err(|e| format!("Failed to send: {}", e))?;

        // Add to local session history
        if let Some(session) = state.session_manager.get_session_by_stream(id).await {
            session
                .write()
                .await
                .add_message(sender_id, text.clone(), MessageType::Text);
        }

        println!("✅ Sent: {}", text);
    } else {
        return Err("No active chat. Use 'call <peer_id>' first".to_string());
    }

    Ok(())
}

async fn send_typing_indicator(state: &AppState) -> Result<(), String> {
    let stream_id = *state.active_stream.read().await;

    if let Some(id) = stream_id {
        let sender_id = state.device.get_id().await.to_string();

        let msg = ChatMessage {
            sender: sender_id,
            content: String::new(),
            timestamp: chrono::Utc::now().timestamp(),
            message_type: MessageType::Typing,
        };

        let data = serde_json::to_vec(&msg).map_err(|e| format!("Failed to serialize: {}", e))?;

        state
            .device
            .send_stream_data(id, data)
            .await
            .map_err(|e| format!("Failed to send typing indicator: {}", e))?;

        println!(" Typing indicator sent");
    } else {
        return Err("No active chat".to_string());
    }

    Ok(())
}

async fn hangup(state: &AppState) -> Result<(), String> {
    let mut lock = state.active_stream.write().await;

    if let Some(id) = lock.take() {
        state
            .device
            .close_stream(id)
            .await
            .map_err(|e| format!("Failed to close stream: {}", e))?;

        println!(" Chat ended");
    } else {
        return Err("No active chat".to_string());
    }

    Ok(())
}

async fn list_sessions(state: &AppState) {
    let sessions = state.session_manager.get_all_sessions().await;

    if sessions.is_empty() {
        println!(" No active sessions");
        return;
    }

    let active_stream = *state.active_stream.read().await;

    println!("\n Active Sessions ({}):\n", sessions.len());
    for session in sessions {
        let is_active = Some(session.stream_id) == active_stream;
        let indicator = if is_active { "→" } else { " " };

        println!("  {} Session: {}", indicator, session.id);
        println!("     Peer: {}", session.peer_id);
        println!("     Messages: {}", session.message_count);
        println!("     Started: {}", format_timestamp(session.started_at));
        println!();
    }
}

async fn show_history(state: &AppState, session_id: Option<String>) -> Result<(), String> {
    let session = if let Some(id) = session_id {
        state
            .session_manager
            .get_session(&id)
            .await
            .ok_or_else(|| format!("Session '{}' not found", id))?
    } else {
        // Use active session
        let stream_id = state
            .active_stream
            .read()
            .await
            .ok_or("No active session. Specify session ID or start a chat")?;

        state
            .session_manager
            .get_session_by_stream(stream_id)
            .await
            .ok_or("Active session not found")?
    };

    let session_guard = session.read().await;

    println!("\n Chat History - Session {}", session_guard.id);
    println!("   Peer: {}", session_guard.peer_id);
    println!("   Messages: {}\n", session_guard.messages.len());

    if session_guard.messages.is_empty() {
        println!("   (no messages yet)");
    } else {
        for (_i, msg) in session_guard.messages.iter().enumerate() {
            let time = format_timestamp(msg.timestamp);
            let icon = match msg.message_type {
                MessageType::Text => "💬",
                MessageType::Typing => "⌨️",
                MessageType::System => "🔔",
                MessageType::Command => "⚙️",
            };

            println!("  {}  [{}] {}: {}", icon, time, msg.sender, msg.content);
        }
    }

    println!();
    Ok(())
}

async fn switch_session(state: &AppState, session_id: String) -> Result<(), String> {
    let session = state
        .session_manager
        .get_session(&session_id)
        .await
        .ok_or_else(|| format!("Session '{}' not found", session_id))?;

    let session_guard = session.read().await;
    let stream_id = session_guard.stream_id;
    drop(session_guard);

    *state.active_stream.write().await = Some(stream_id);

    println!(" Switched to session '{}'", session_id);
    Ok(())
}

async fn broadcast_message(state: &AppState, message: String) -> Result<(), String> {
    let sessions = state.session_manager.get_all_sessions().await;

    if sessions.is_empty() {
        return Err("No active sessions to broadcast to".to_string());
    }

    let sender_id = state.device.get_id().await.to_string();
    let mut sent = 0;

    for session_info in sessions {
        let msg = ChatMessage {
            sender: sender_id.clone(),
            content: message.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            message_type: MessageType::Text,
        };

        if let Ok(data) = serde_json::to_vec(&msg)
            && state
                .device
                .send_stream_data(session_info.stream_id, data)
                .await
                .is_ok()
        {
            sent += 1;
        }
    }

    println!(" Broadcast to {} session(s)", sent);
    Ok(())
}

async fn show_stats(state: &AppState) {
    let sessions = state.session_manager.get_all_sessions().await;
    let subscriptions = state.subscriptions.read().await;

    println!("\n Statistics:\n");
    println!("  Active Sessions: {}", sessions.len());
    println!("  Subscriptions: {}", subscriptions.len());

    let total_messages: usize = sessions.iter().map(|s| s.message_count).sum();
    println!("  Total Messages: {}", total_messages);

    if let Ok(peers) = state.device.get_peers().await {
        println!("  Connected Peers: {}", peers.len());
    }

    println!();
}

fn format_timestamp(ts: i64) -> String {
    use chrono::{DateTime, Local, TimeZone};
    let dt: DateTime<Local> = Local.timestamp_opt(ts, 0).unwrap();
    dt.format("%H:%M:%S").to_string()
}

async fn on_started(device: AviDevice, peer_id: String, state: Arc<AppState>) {
    println!("\n🟢 Node started: {}\n", peer_id);

    // Auto-subscribe to global
    device
        .subscribe("global", move |from, topic, data| {
            let msg = String::from_utf8_lossy(&data);
            println!("\n [{}] {} → {}", topic, from, msg);
            print!("> ");
            let _ = io::stdout().flush();
        })
        .await
        .expect("Failed to subscribe to global");

    // Subscribe to personal speaker topic
    let speaker_topic = format!("speak/{}/text", device.get_id().await);
    device
        .subscribe(&speaker_topic, move |_from, _topic, data| {
            let msg = String::from_utf8_lossy(&data);
            println!("\n Speaker: {}", msg);
            print!("> ");
            let _ = io::stdout().flush();
        })
        .await
        .expect("Failed to subscribe to speaker topic");

    // Update subscription tracking
    state
        .subscriptions
        .write()
        .await
        .insert("global".to_string(), "global".to_string());
    state
        .subscriptions
        .write()
        .await
        .insert(speaker_topic.clone(), speaker_topic);

    print!("> ");
    io::stdout().flush().unwrap();
}
