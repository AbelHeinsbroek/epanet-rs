use std::collections::HashMap;

pub struct Network {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,

    pub node_map: HashMap<Box<str>, usize>,
    pub link_map: HashMap<Box<str>, usize>,
}

impl Network {
  pub fn new() -> Self {
    Network {
      nodes: Vec::new(),
      links: Vec::new(),
      node_map: HashMap::new(),
      link_map: HashMap::new(),
    }
  }
  pub fn add_node(&mut self, node: Node) -> Result<(), String> {
    if self.node_map.contains_key(&node.id) {
      return Err(format!("Node {} already exists", node.id));
    }
    self.node_map.insert(node.id.clone(), self.nodes.len());
    self.nodes.push(node);
    Ok(())
  }
  pub fn add_link(&mut self, link: Link) -> Result<(), String> {
    if self.link_map.contains_key(&link.id) {
      return Err(format!("Link {} already exists", link.id));
    }
    self.link_map.insert(link.id.clone(), self.links.len());
    self.links.push(link);
    Ok(())
  }
}

pub enum NodeType {
    Reservoir,
    Demand,
    Junction { basedemand: f64 },
}

pub enum LinkType {
    Pipe { diameter: f64, length: f64, roughness: f64 },
    Pump,
    Valve,
}
pub struct NodeResult {
  pub head: f64
}

impl NodeResult {
  pub fn new() -> Self {
    NodeResult {
      head: 0.0,
    }
  }
}

#[derive(Default)]
pub struct LinkResult {
  pub flow: f64,
}

#[derive(Default)]
pub struct CSCIndex {
  pub diag_u: Option<usize>,      // CSC index for J[u,u]
  pub diag_v: Option<usize>,      // CSC index for J[v,v]
  pub off_diag_uv: Option<usize>, // CSC index for J[u,v]
  pub off_diag_vu: Option<usize>, // CSC index for J[v,u]
}

pub struct Node {
    pub id: Box<str>,
    pub node_type: NodeType,
    pub elevation: f64,

    pub result: NodeResult,
}

pub struct Link {
  pub id: Box<str>,
  pub link_type: LinkType,
  pub start_node: usize,
  pub end_node: usize,

  pub resistance: f64,

  pub result: LinkResult,

  pub csc_index: CSCIndex,
}