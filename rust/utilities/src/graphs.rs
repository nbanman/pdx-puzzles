use crate::structs::store::Store;
use num_traits::{One, Zero};
use rustc_hash::FxHashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::{collections::VecDeque, hash::Hash};

pub fn bfs<S, C, FN, FS>(start: S, edges: FN, end_condition: FS) -> PathInfo<S, C>
where
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
    FN: Fn(EdgeInfo<C>, &S) -> Vec<S>,
    FS: Fn(EdgeInfo<C>, &S) -> bool,
{
    let frontier: VecDeque<(EdgeInfo<C>, S)> = VecDeque::new();
    traverse_unweighted(start, edges, end_condition, frontier)
}

pub fn dfs<S, C, FN, FS>(start: S, edges: FN, end_condition: FS) -> PathInfo<S, C>
where
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
    FN: Fn(EdgeInfo<C>, &S) -> Vec<S>,
    FS: Fn(EdgeInfo<C>, &S) -> bool,
{
    let frontier: Vec<(EdgeInfo<C>, S)> = Vec::new();

    traverse_unweighted(start, edges, end_condition, frontier)
}

fn traverse_unweighted<'a, S, C, FN, FS, FR>(
    start: S,
    edges: FN,
    end_condition: FS,
    mut frontier: FR,
) -> PathInfo<S, C>
where
    S: Debug + Eq + Hash + Clone + 'a,
    C: Debug + Zero + One + Copy + Hash + Eq + 'a,
    FN: Fn(EdgeInfo<C>, &S) -> Vec<S>,
    FS: Fn(EdgeInfo<C>, &S) -> bool,
    FR: Frontier<(EdgeInfo<C>, S)>,
{
    frontier.push_node((
        EdgeInfo {
            cost: C::zero(),
            parent: None,
        },
        start,
    ));
    let mut visited = Store::new();

    while let Some((info, node)) = frontier.pop_node() {
        // add to visited
        let Some(id) = visited.assign(node.clone(), info) else {
            continue;
        };

        // check end condition and exit if successful
        if end_condition(info, &node) {
            return PathInfo {
                nodes: visited,
                end_index: Some(id),
            };
        }

        // add connected nodes to frontier
        let cost = info.cost + C::one();
        for edge in edges(info, &node).into_iter() {
            frontier.push_node((
                EdgeInfo {
                    cost,
                    parent: Some(id),
                },
                edge,
            ));
        }
    }
    PathInfo {
        nodes: visited,
        end_index: None,
    }
}

pub fn dijkstra<S, C, FN, FS>(
    start: S,
    edges: FN,
    end_condition: FS,
) -> PathInfo<S, C>
where
    S: Debug + Eq + Hash + Ord + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq + Ord,
    FN: Fn(EdgeInfo<C>, &S) -> Vec<(S, C)>,
    FS: Fn(EdgeInfo<C>, &S) -> bool,
{
    let frontier: BinaryHeap<Reverse<(EdgeInfo<C>, S)>> = BinaryHeap::new();
    traverse_weighted(start, edges, end_condition, frontier)
}

fn traverse_weighted<S, C, FN, FS>(
    start: S,
    edges: FN,
    end_condition: FS,
    mut frontier: BinaryHeap<Reverse<(EdgeInfo<C>, S)>>
) -> PathInfo<S, C>
where
    C: Copy + Debug + Eq + Hash + One + Ord + Zero,
    FN: Fn(EdgeInfo<C>, &S) -> Vec<(S, C)>,
    FS: Fn(EdgeInfo<C>, &S) -> bool,
    S: Clone + Debug + Eq + Hash + Ord,
{
    let mut costs = FxHashMap::default();
    costs.insert(start.clone(), C::zero());

    frontier.push_node(Reverse((
        EdgeInfo {
            cost: C::zero(),
            parent: None,
        },
        start
    )));
    let mut visited = Store::new();
    while let Some(Reverse((info, node))) = frontier.pop_node() {
        let Some(id) = visited.assign(node.clone(), info) else {
            continue;
        };

        // check end condition and exit if successful
        if end_condition(info, &node) {
            return PathInfo {
                nodes: visited,
                end_index: Some(id),
            };
        }

        // add connected nodes to frontier
        for (edge_state, edge_cost) in edges(info, &node).into_iter() {
            let alternate_cost = info.cost + edge_cost;
            let cost = *costs.get(&edge_state).unwrap_or(&(alternate_cost + C::one()));
            if alternate_cost < cost {
                costs.insert(edge_state.clone(), alternate_cost);
                frontier.push_node(Reverse((
                    EdgeInfo {
                        cost: alternate_cost,
                        parent: Some(id),
                    },
                    edge_state,
                )));
            }
        }
    }
    PathInfo {
        nodes: visited,
        end_index: None,
    }
}

pub fn no_end_condition<T, U>(_: T, _: &U) -> bool {
    false
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeInfo<C>
where
    C: Debug + Zero + One + Copy + Hash + Eq,
{
    pub cost: C,
    pub parent: Option<usize>,
}

#[derive(Debug, Clone)]
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
        let cost = self
            .nodes
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
                (state, edge_info) = self
                    .nodes
                    .get_entry(valid_id)
                    .expect("end_index should always be in the Indexer");
                let step = Step {
                    id: valid_id,
                    state: state.clone(),
                    cost: edge_info.cost,
                    parent: edge_info.parent.map(|id| {
                        self.nodes
                            .get_entry(id)
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

#[derive(Clone, Debug)]
pub struct Step<S, C>
where
    S: Debug + Eq + Hash + Clone,
    C: Debug + Zero + One + Copy + Hash + Eq,
{
    pub id: usize,
    pub state: S,
    pub cost: C,
    pub parent: Option<S>,
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

impl<T> Frontier<T> for BinaryHeap<T>
where
    T: Ord,
{
    fn push_node(&mut self, value: T) { self.push(value); }
    fn pop_node(&mut self) -> Option<T> { self.pop() }
}

#[test]
fn bfs_test() {
    use ordered_float::OrderedFloat;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(OrderedFloat<f32>, OrderedFloat<f32>);

    impl Pos {
        fn successors(&self) -> Vec<Pos> {
            vec![
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
            ]
            .into_iter()
            .map(|(x, y)| Pos(self.0 + x as f32, self.1 + y as f32))
            .collect()
        }
    }
    static GOAL: Pos = Pos(OrderedFloat(4.0), OrderedFloat(6.0));
    let result = bfs(
        Pos(OrderedFloat(1.0), OrderedFloat(1.0)),
        |_, p| p.successors(),
        |_, p: &Pos| *p == GOAL,
    );
    assert_eq!(result.steps(), Some(OrderedFloat(4.0)))
}

#[test]
fn dfs_test() {
    let result = dfs(
        1,
        |_: EdgeInfo<usize>, &n| {
            let edges = vec![n + 1, n * n].into_iter().filter(|&x| x <= 17);
            edges.collect()
        },
        |_, _| false // |n: &i32, _| *n == 17,
    );
    assert_eq!(
        result
            .path(4)
            .iter()
            .map(|step| step.state)
            .collect::<Vec<_>>(),
        vec![1, 2, 4, 16, 17]
    );
}
