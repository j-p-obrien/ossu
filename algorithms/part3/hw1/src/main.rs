use hw1::scheduling::JobList;
use hw1::spanning_tree::*;

fn main() {
    let mut jobs = JobList::parse_job_list("jobs.txt");

    jobs.schedule_jobs_additive();
    let weighted_completion_times = jobs.weighted_completion_times();
    let completion_times_sum: u64 = weighted_completion_times.iter().sum();
    println!(
        "The sum of the weighted completion times is: {}",
        completion_times_sum
    );

    jobs.schedule_jobs_optimal();
    let weighted_completion_times = jobs.weighted_completion_times();
    let completion_times_sum: u64 = weighted_completion_times.iter().sum();
    println!(
        "The sum of the weighted completion times is: {}",
        completion_times_sum
    );

    let graph = AdjacencyList::parse_file("edges.txt");
    if let Some(edges) = graph.prims_mst() {
        let total_cost: i32 = edges.iter().map(|edge| edge.cost).sum();
        println!("Total cost of the spanning tree is: {}", total_cost)
    }
}
