use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::schema::{node::NodeSchema, workflow::WorkflowSchema};

struct GraphNode {
    pub node_id: String,
    pub node_context: NodeSchema,
    pub next: Vec<Rc<RefCell<GraphNode>>>,
}

fn build_graph(
    node: &Rc<RefCell<GraphNode>>,
    edges: &HashMap<String, Vec<String>>,
    nodes: &HashMap<String, NodeSchema>,
) -> Result<(), String> {
    let next_edges = if let Some(edges) = edges.get(&node.borrow().node_id) {
        edges
    } else {
        return Ok(());
    };
    let mut next_nodes = vec![];
    for next_node_id in next_edges.iter() {
        let node = nodes
            .get(next_node_id)
            .ok_or_else(|| format!("connection references missing node '{}'", next_node_id))?;

        let rc_node = Rc::new(RefCell::new(GraphNode {
            node_id: next_node_id.clone(),
            node_context: node.clone(),
            next: vec![],
        }));
        build_graph(&rc_node.clone(), edges, nodes)?;

        next_nodes.push(rc_node);
    }
    node.borrow_mut().next = next_nodes;
    Ok(())
}

pub struct WorkflowRunner {
    graph: Vec<Rc<RefCell<GraphNode>>>,
}

impl WorkflowRunner {
    pub fn create(workflow: WorkflowSchema) -> Result<Self, String> {
        let mut graph: Vec<Rc<RefCell<GraphNode>>> = vec![];
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        let mut nodes: HashMap<String, NodeSchema> = HashMap::new();

        for (i, node) in workflow.nodes.into_iter().enumerate() {
            let key = format!("node-{i}");
            nodes.insert(key, node);
        }

        for edge in workflow.connections.into_iter() {
            if !nodes.contains_key(&edge.from) {
                return Err(format!(
                    "connection references missing node '{}'",
                    edge.from
                ));
            }
            if !nodes.contains_key(&edge.to) {
                return Err(format!("connection references missing node '{}'", edge.to));
            }

            let entry = edges.entry(edge.from).or_insert_with(Vec::new);
            entry.push(edge.to);
        }

        for (key, node) in nodes.iter() {
            if node.action_type == "Start" {
                let node = GraphNode {
                    node_id: key.clone(),
                    node_context: node.clone(),
                    next: vec![],
                };
                graph.push(Rc::new(RefCell::new(node)));
            }
        }

        for node in graph.iter() {
            build_graph(node, &edges, &nodes)?;
        }

        Ok(Self { graph })
    }

    pub fn run() -> Result<(), String> {
        // TODO: run workflow
        Ok(())
    }
}