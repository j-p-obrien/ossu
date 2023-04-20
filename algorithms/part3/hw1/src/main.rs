use hw1::scheduling::JobList;


fn main() {
    let mut jobs = JobList::parse_job_list("jobs.txt");
    jobs.schedule_jobs_additive();
    let weighted_completion_times = jobs.weighted_completion_times();
    let completion_times_sum: u32 = weighted_completion_times.iter().sum();
    println!("The sum of the weighted completion times is: {}", completion_times_sum);
    
    jobs.schedule_jobs_optimal();
    let weighted_completion_times = jobs.weighted_completion_times();
    let completion_times_sum: u32 = weighted_completion_times.iter().sum();
    println!("The sum of the weighted completion times is: {}", completion_times_sum);

    

}
