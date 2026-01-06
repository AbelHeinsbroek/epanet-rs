use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::network::*;

#[derive(Debug)]
enum ReadState {
  None,
  Junctions,
  Pipes,
  Reservoirs,
}

impl Network {
  pub fn from_inp(inp: &str) -> Self {

    let mut network = Network::new();

    let mut state = ReadState::None;

    let file = File::open(inp).unwrap();
    let mut reader = BufReader::new(file);
    let mut line_buffer = String::with_capacity(512);

    while reader.read_line(&mut line_buffer).unwrap() > 0 {
      let line = line_buffer.trim();

      if line.starts_with(";") || line.is_empty() {
        // skip comment and empty lines
      }
      else if line.starts_with("[") {
        state = match line {
          "[JUNCTIONS]" => ReadState::Junctions,
          "[PIPES]" => ReadState::Pipes,
          "[RESERVOIRS]" => ReadState::Reservoirs,
          _ => ReadState::None,
        }
      }
      else {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match state {
          ReadState::Junctions => {
            // read the junction data
            let id = parts[0].trim().into();
            // read the elevation (optional, default 0.0)
            let elevation = parts.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            // read the demand (optional, default 0.0)
            let demand = parts.get(2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);

            // add the node to the network
            network.add_node(Node {
              id,
              elevation,
              node_type: NodeType::Junction { basedemand: demand },
              result: NodeResult::new(),
            }).unwrap();
          }
          ReadState::Pipes => {
            let id = parts[0].trim().into();
            // read the start node
            let start_node: Box<str> = parts[1].trim().into();
            // read the end node
            let end_node: Box<str> = parts[2].trim().into();
            // read the diameter
            let diameter = parts[3].parse::<f64>().unwrap();
            // read the length
            let length = parts[4].parse::<f64>().unwrap();
            // read the roughness
            let roughness = parts[5].parse::<f64>().unwrap();
            // create the link
            let start_node_index = *network.node_map.get(&start_node).unwrap();
            let end_node_index = *network.node_map.get(&end_node).unwrap();

            let _ = network.add_link(Link {
              id,
              start_node: start_node_index,
              end_node: end_node_index,
              resistance: 0.0,
              link_type: LinkType::Pipe { diameter, length, roughness },
              result: LinkResult::default(),
              csc_index: CSCIndex::default(),
            });
          }
          ReadState::Reservoirs => {
            let id = parts[0].trim().into();
            // read the elevation
            let elevation = parts[1].parse::<f64>().unwrap();
            // add the node to the network
            let _ = network.add_node(Node {
              id,
              elevation,
              node_type: NodeType::Reservoir,
              result: NodeResult::new(),
            });
          }
          ReadState::None => {
            // skip unknown state
          }
        }
      }
      // clear the line buffer
      line_buffer.clear();
    }

    return network;
  }
}
