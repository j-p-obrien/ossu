use hw1::scheduling::*;


fn main() {
    let mut jobs = JobList::parse_job_list("jobs.txt");
    jobs.schedule_jobs_additive();
    let completion_times = jobs.completion_times();
    let completion_times_sum: u32 = completion_times.iter().sum();
    println!("The sum of the completion times is: {}", completion_times_sum)
}
