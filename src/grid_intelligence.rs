use std::sync::{Mutex, OnceLock};

struct GridNode {
    id: String,
    status: String,
    threat_weight: u32,
}

fn node_registry() -> &'static Mutex<Vec<GridNode>> {
    static REGISTRY: OnceLock<Mutex<Vec<GridNode>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn map_nodes() {
    println!("[🌐 GRID] Autonomous Network Mesh Topology Mapping Activated.");
}

pub fn sync_node_health(node_id: &str, detected_threats: u32) {
    let mut registry = node_registry().lock().unwrap();
    
    if let Some(node) = registry.iter_mut().find(|n| n.id == node_id) {
        node.threat_weight = detected_threats;
        node.status = if detected_threats > 50 { "CRITICAL".to_string() } else { "HEALTHY".to_string() };
    } else {
        registry.push(GridNode {
            id: node_id.to_string(),
            status: "ACTIVE".to_string(),
            threat_weight: detected_threats,
        });
    }
    println!("[🌐 GRID] Synced Cluster Node [{}]. Global Grid Threat Weight: {} threats.", node_id, detected_threats);
}
