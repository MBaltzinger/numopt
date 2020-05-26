use std::collections::HashMap;

use crate::model::node::Node;
use crate::model::node_diff::NodeDiff;
use crate::model::constant::ConstantScalar;

pub struct NodeStdProp {
    pub affine: bool,
    pub a: HashMap<Node, f64>,
    pub b: f64,
}

pub struct NodeStdComp {
    pub phi: Node,
    pub gphi: Vec<(Node, Node)>,
    pub hphi: Vec<(Node, Node, Node)>,
    pub prop: NodeStdProp,
}

pub trait NodeStd {
    fn properties(&self) -> NodeStdProp {
        NodeStdProp {
            affine: false,
            a: HashMap::new(),
            b: 0.,
        }
    }
    fn components(&self) -> NodeStdComp {
        NodeStdComp {
            phi: ConstantScalar::new(0.),
            gphi: Vec::new(),
            hphi: Vec::new(),
            prop: self.properties(),
        }
    }
}

impl NodeStd for Node {

    fn properties(&self) -> NodeStdProp {
        match self {
            Node::ConstantScalar(x) => {
                NodeStdProp {
                    affine: true,
                    a: HashMap::new(),
                    b: x.value(),
                }
            },
            Node::VariableScalar(_x) => {
                let mut a: HashMap<Node, f64> = HashMap::new();
                a.insert(self.clone(), 1.);
                NodeStdProp {
                    affine: true,
                    a: a,
                    b: 0.,
                }
            },
            Node::FunctionAdd(x) => x.properties(),
            Node::FunctionCos(x) => x.properties(),
            Node::FunctionDiv(x) => x.properties(),
            Node::FunctionMul(x) => x.properties(),
            Node::FunctionSin(x) => x.properties(),
        }
    }

    fn components(&self) -> NodeStdComp {

        let phi = self.clone();
        let mut gphi: Vec<(Node, Node)> = Vec::new();
        let mut hphi: Vec<(Node, Node, Node)> = Vec::new();
        let prop = self.properties();

        // Affine
        if prop.affine {
            for (key, val) in prop.a.iter() {
                gphi.push((key.clone(), ConstantScalar::new(*val)));
            }
        }

        // Not affine
        else {
            let vars: Vec<&Node> = prop.a.keys().collect();
            let derivs = self.derivatives(&vars);
            for (i, var1) in vars.iter().enumerate() {
                let d = derivs.get(var1).unwrap();
                gphi.push(((*var1).clone(), d.clone()));
                let dvars: Vec<&Node> = vars.iter()
                                            .enumerate()
                                            .filter(|&(k,_)| k >= i)
                                            .map(|(_,v)| *v)
                                            .collect();
                let dderivs = d.derivatives(&dvars);
                for var2 in dvars.iter() {
                    let dd = dderivs.get(&var2).unwrap();
                    if !dd.is_constant_with_value(0.) {
                        hphi.push(((*var1).clone(), (*var2).clone(), dd.clone()));
                    }
                }                     
            }
        }

        // Return
        NodeStdComp {
            phi: phi,
            gphi: gphi,
            hphi: hphi,
            prop: prop
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::node_func::NodeFunc;
    use crate::model::variable::VariableScalar;

    #[test]
    fn components_affine() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        // Affine
        let z1 = 7.*&x + 10.*&y + 5.;
        let c1 = z1.components();

        assert!(c1.prop.affine);
        assert_eq!(c1.phi, z1);
        assert_eq!(c1.gphi.len(), 2);
        for (v, e) in c1.gphi.iter() {
            if v == &x {
                assert!(e.is_constant_with_value(7.));
            }
            else if v == &y {   
                assert!(e.is_constant_with_value(10.));
            }
            else {
                panic!("invalid variable");
            }
        }
        assert_eq!(c1.hphi.len(), 0);
    }

    #[test]
    fn components_not_affine() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        // Not affine
        let z2 = 7.*&x.cos() + 10.*&y*&x + 5.;
        let c2 = z2.components();
        
        assert!(!c2.prop.affine);
        assert_eq!(c2.phi, z2);
        assert_eq!(c2.gphi.len(), 2);
        for (v, e) in c2.gphi.iter() {
            if v == &x {
                assert_eq!(format!("{}", e), "10*y + 7*-1*sin(x)");
            }
            else if v == &y {
                assert_eq!(format!("{}", e), "x*10");
            }
            else {
                panic!("invalid variable");
            }
        }
        assert_eq!(c2.hphi.len(), 2);
        for (v1, v2, e) in c2.hphi.iter() {
            if v1 == &x && v2 == &x {
                assert_eq!(format!("{}", e), "-7*cos(x)");
            }
            else if v1 == &x && v2 == &y {
                assert!(e.is_constant_with_value(10.));
            }
            else if v1 == &y && v2 == &x {
                assert!(e.is_constant_with_value(10.));
            }
            else {
                panic!("invalid variable");
            }
        }
    }
}