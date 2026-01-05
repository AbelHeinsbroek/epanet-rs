#[derive(Clone, Debug)]
struct Node {
  id: String,
  x: f64,
  y: f64,
  elevation: f64,
  demand: f64,
  head: f64,
  is_fixed: bool // fixed head (reservoir node)
}

#[derive(Clone, Debug)]
struct Link {
  id: String,
  start: usize,
  end: usize,
  diameter: f64,
  roughness: f64,
  length: f64,
  flow: f64
}

#[derive(Clone, Debug)]
struct Network {
  nodes: Vec<Node>,
  links: Vec<Link>,
}

impl Network {


  /// Main solve loop
  fn solve(&mut self, max_iter: usize, accuracy: f64) {

    // 1. Initialize flows in links
    for link in &mut self.links {
      link.flow = 1.0;
    }

  }
}

fn main() {
  let network = generate_grid(4);



}

fn generate_grid(dim: u32) -> Network {

  let mut nodes : Vec<Node> = Vec::new();
  let mut links : Vec<Link> = Vec::new();

  for x in 0..dim {
    for y in 0..dim {
      let is_fixed = x == 0 && y == 0;
      let head = if is_fixed { 100.0 } else { 0.0 };

      nodes.push(Node {
        id: format!("N{}-{}", x, y),
        x: x as f64,
        y: y as f64,
        elevation: 0.0,
        demand: x as f64 + y as f64,
        head: head,
        is_fixed: is_fixed
      })
    }
  }
  let mut link_id = 0;

  for x in 0..dim {
    for y in 0..dim {

      let idx = x * dim + y;

      // add horizontal link
      if x < dim - 1 {
        links.push(generate_link(link_id, idx as usize, (idx+dim) as usize));
        link_id+=1;
      }
      // add vertical link
      if y < dim - 1 {
        links.push(generate_link(link_id, idx as usize, (idx+1) as usize));
        link_id+=1;
      }



    }
  }

  return Network {
    nodes, links
  };

}

fn generate_link(link_id : i32, from : usize, to : usize) -> Link {
  Link {
    id: format!("L{}", link_id),
    start: from,
    end: to,
    length: 100.0,
    diameter: 500.0,
    roughness: 100.0,
    flow: 0.0,
  }
}