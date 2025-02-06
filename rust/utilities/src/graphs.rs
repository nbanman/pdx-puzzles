use std::fmt::Debug;
use std::{collections::VecDeque, hash::Hash};

use num_traits::{Zero, One};

use crate::structs::store::Store;

pub fn bfs<S, C, FN, E, FS>(
    start: S,
    edges: FN,
    end_condition: Option<FS>,
) -> PathInfo<S, C>
where 
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
    FN: Fn(&S, EdgeInfo<C>) -> E,
    E: IntoIterator<Item = S>,
    FS: Fn(&S, EdgeInfo<C>) -> bool,
{
    let frontier: VecDeque<(S, EdgeInfo<C>)> = VecDeque::new();
    traverse(start, edges, end_condition, frontier)
}

pub fn dfs<S, C, FN, E, FS>(
    start: S,
    edges: FN,
    end_condition: Option<FS>,
) -> PathInfo<S, C>
where 
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
    FN: Fn(&S, EdgeInfo<C>) -> E,
    E: IntoIterator<Item = S>,
    FS: Fn(&S, EdgeInfo<C>) -> bool,
{
    let frontier: Vec<(S, EdgeInfo<C>)> = Vec::new();
    traverse(start, edges, end_condition, frontier)
}

pub fn dijkstra<S, C, FN, E, FS>(
    start: S,
    edges: FN,
    end_condition: Option<FS>,
) -> PathInfo<S, C>
where 
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
    FN: Fn(&S, EdgeInfo<C>) -> E,
    E: IntoIterator<Item = S>,
    FS: Fn(&S, EdgeInfo<C>) -> bool,
{
    let frontier: VecDeque<(S, EdgeInfo<C>)> = VecDeque::new();
    traverse(start, edges, end_condition, frontier)
}

fn traverse<'a, S, C, FN, E, FS, FR>(
    start: S,
    edges: FN, 
    end_condition: Option<FS>, 
    mut frontier: FR,
) -> PathInfo<S, C> 
where 
    S: Debug + Eq + Hash + Clone + 'a, 
    C: Debug + Zero + One + Copy + Hash + Eq + 'a, 
    FN: Fn(&S, EdgeInfo<C>) -> E,
    E: IntoIterator<Item = S>,
    FS: Fn(&S, EdgeInfo<C>) -> bool,
    FR: Frontier<(S, EdgeInfo<C>)>
{
    frontier.push_node((start, EdgeInfo { cost: C::zero(), parent: None }));
    let mut visited = Store::new();

    while let Some((node, info)) = frontier.pop_node() {
        // add to visited
        let Some(id) = visited.assign(node.clone(), info) else { continue; };
    
        // check end condition and exit if successful
        if let Some(end_condition) = &end_condition {
            if end_condition(&node, info) { 
                return PathInfo { nodes: visited, end_index: Some(id) };
            }
        }
    
        // add connected nodes to frontier
        let cost = info.cost + C::one();
        for edge in edges(&node, info).into_iter() {
            frontier.push_node((edge, EdgeInfo { cost, parent: Some(id) }));
        }
    }
    PathInfo { nodes: visited, end_index: None }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct EdgeInfo<C>
where 
    C: Debug + Zero + One + Copy + Hash + Eq,
{
    cost: C,
    parent: Option<usize>,
}

#[derive(Debug)]
pub struct PathInfo<S, C> 
where 
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
{
    pub nodes: Store<S, EdgeInfo<C>>,
    pub end_index: Option<usize>,
}

impl<S, C> PathInfo<S, C>
where 
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
{
    pub fn steps(&self) -> Option<C> {
        let end_index = self.end_index?;
        let cost = self.nodes
            .get_entry(end_index)
            .expect("end_index should always be in the Store")
            .1
            .cost;
        Some(cost)
    }

    pub fn path(&self, id: usize) -> Vec<Step<S, C>> {
        let mut steps = Vec::new();
        let mut state: &S;
        let mut edge_info: &EdgeInfo<C>;
        let mut id = Some(id);
        loop {
            if let Some(valid_id) = id {
                (state, edge_info) = self.nodes
                .get_entry(valid_id)
                .expect("end_index should always be in the Indexer");
                let step = Step {
                    id: valid_id,
                    state: state.clone(),
                    cost: edge_info.cost,
                    parent: edge_info.parent.map(|id| {
                        self.nodes.get_entry(id) 
                            .expect("Parent index should always be valid.")
                            .0
                            .clone()
                    }),
                };
                steps.push(step);
                id = edge_info.parent;
            } else {
                break;
            }
        }
        steps.into_iter().rev().collect()
    }
}

pub struct Step<S, C>
where 
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
{
    id: usize,
    state: S,
    cost: C,
    parent: Option<S>,
}

trait Frontier<T> {
    fn push_node(&mut self, value: T);
    fn pop_node(&mut self) -> Option<T>;
}

impl<T> Frontier<T> for Vec<T> {
    fn push_node(&mut self, value: T) {
        self.push(value);
    }

    fn pop_node(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T> Frontier<T> for VecDeque<T> {
    fn push_node(&mut self, value: T) {
        self.push_back(value);
    }

    fn pop_node(&mut self) -> Option<T> {
        self.pop_front()
    }
}

#[test]
fn bfs_test() {
    use ordered_float::OrderedFloat;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(OrderedFloat<f32>, OrderedFloat<f32>);

    impl Pos {
        fn successors(&self) -> Vec<Pos> {
            vec![
                (1,2), (1,-2), (-1,2), (-1,-2),
                (2,1), (2,-1), (-2,1), (-2,-1)
                ]
                .into_iter()
                .map(|(x, y)| Pos(self.0 + x as f32, self.1 + y as f32)).collect()
        }
    }
    static GOAL: Pos = Pos(OrderedFloat(4.0), OrderedFloat(6.0));
    let result = bfs(
        Pos(OrderedFloat(1.0), OrderedFloat(1.0)),
        |p, _| p.successors(),
        Some(|p: &Pos, _| *p == GOAL)
    );
    assert_eq!(result.steps(), Some(OrderedFloat(4.0)))
}

#[test]
fn dfs_test() {
    let result = dfs(
        1,
        |&n, _: EdgeInfo<usize>| vec![n+1, n*n].into_iter().filter(|&x| x <= 17),
        Some(|n: &i32, _| *n == 17)
    );
    assert_eq!(
        result.path(4).iter().map(|step| step.state).collect::<Vec<_>>(),
        vec![1, 2, 4, 16, 17]
    );
}

