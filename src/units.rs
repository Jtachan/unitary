//! Module with the declaration for a single DigitalUnit instance.

use std::collections::HashMap;

/// Node containing the parts of a single unit. For example, the composed unit "km / h"
/// (kilometers per hour) is composed by two nodes: 'kilometers' and 'hour'
pub struct UnitNode {
    prefix: String,
    base: String,
    exponent: i32,
}

pub struct CompositeUnit {
    nodes: Vec<UnitNode>,
    dimensions: HashMap<String, i32>,
    scale_to_base: f32,
}

// impl CompositeUnit {
//     pub fn from_nodes(nodes: Vec<UnitNode>) -> Self {
//         let mut dimensions: HashMap<String, i32> = HashMap::new();
//         let mut scale_to_base: f32 = 1.0;
//
            // TODO: Need to implement the "prefix.factor()".
//         for node in &nodes {
//             *dimensions.entry(node.base.to_string()).or_insert(0i32) += node.exponent;
//             scale_to_base *= node.prefix.factor().powi(node.exponent);
//         }
//
//         CompositeUnit {
//             nodes, dimensions, scale_to_base
//         }
//     }
// }
