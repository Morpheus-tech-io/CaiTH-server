use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::ws::Message;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ClientState {
    pub user_id: Uuid,
    pub match_id: Option<Uuid>,
    pub last_ping: std::time::Instant,
    pub sender: mpsc::UnboundedSender<Message>,
}

#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<Uuid, ClientState>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_sender(&self, conn_id: &Uuid) -> Option<mpsc::UnboundedSender<Message>> {
        let connections = self.connections.read().await;
        connections.get(conn_id).map(|state| state.sender.clone())
    }

    pub async fn add_connection(&self, conn_id: Uuid, user_id: Uuid, sender: mpsc::UnboundedSender<Message>) {
        let state = ClientState {
            user_id,
            match_id: None,
            last_ping: std::time::Instant::now(),
            sender,
        };
        
        let mut connections = self.connections.write().await;
        connections.insert(conn_id, state);
    }

    pub async fn remove_connection(&self, conn_id: &Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(conn_id);
    }

    pub async fn get_connection(&self, conn_id: &Uuid) -> Option<ClientState> {
        let connections = self.connections.read().await;
        connections.get(conn_id).cloned()
    }
}