use hw1::{self, edge_list::EdgeList};

fn main() {
    let hw1file = "hw1_SCC.txt";
    let mut edgelist = EdgeList::parse_edge_list(hw1file);
    let scc = edgelist.scc();
    let mut scc_sizes: Vec<i32> = scc.iter()
        .map(|x| x.len() as i32)
        .collect();
    scc_sizes.sort_unstable_by_key(|x| -x);
    println!("{:?}", &scc_sizes[0..5])
}
