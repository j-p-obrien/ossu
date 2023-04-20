use std::{fs, str::FromStr, cmp::Ordering};

// Struct that holds the weight and length of the job to be scheduled.
#[derive(Debug, Eq, PartialEq)]
pub struct Job {
    pub weight: u32,
    pub length: u32
}

#[derive(Debug)]
pub struct ParseJobError;

impl Job {
    // Computes weight - length
    pub fn additive_priority(&self) -> i32 {
        self.weight as i32 - self.length as i32
    }

    // computes weight/length
    pub fn multiplicative_priority(&self) -> f32 {
        self.weight as f32 / self.length as f32
    }
}

// Implement FromStr so we can turn a line from the file into a Job.
impl FromStr for Job {
    type Err = ParseJobError;

    fn from_str(line: &str) -> Result<Job, ParseJobError> {
        let job_data: Vec<Result<u32, _>> = line.split_whitespace()
            .map(|num| num.parse())
            .collect();

        if let [Ok(weight), Ok(length)] = job_data[..] { 
            return Ok(Job { weight , length })
        } 
        Err(ParseJobError)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct JobList(pub Vec<Job>);

impl JobList {
    // Parses a list of jobs from the file given by path. Returns the job
    // list in the order they appear.
    pub fn parse_job_list(path: &str) -> JobList {
        let job_file = fs::read_to_string(path)
            .expect("Couldn't read file");

        let mut line_iter = job_file.lines();
        // skip first iteration; it contains only the number of jobs.
        line_iter.next();
        let job_list: Vec<Job> = line_iter
            .map(|line| Job::from_str(line).unwrap())
            .collect();

        return JobList(job_list)
    }

    // Sorts the jobs in decreasing order of additive job cost.
    pub fn schedule_jobs_additive(&mut self) {
        let JobList(job_list) = self;

        // create closure for sort_by function. Higher priority jobs are scheduled first. 
        // If priority is tied, job with higher weight is scheduled first.
        let additive_order = |i: &Job, j: &Job| -> Ordering {
            let i_priority = i.additive_priority();
            let j_priority = j.additive_priority();

            if i_priority == j_priority {
                return j.weight.cmp(&i.weight)
            } else {
                return j_priority.cmp(&i_priority)
            }
        };

        job_list.sort_by(additive_order)
    }

    // Sorts Jobs by decreasing values of weight/length, the optimal ordering.
    pub fn schedule_jobs_optimal(&mut self) {
        let JobList(job_list) = self;
        
        let optimal_order = |i: &Job, j: &Job| -> Ordering {
            let i_priority = i.multiplicative_priority();
            let j_priority = j.multiplicative_priority();

            if let Some(order) = j_priority.partial_cmp(&i_priority) {
                return order
            } else {
                panic!("Prob some kind of division by 0 error.")
            }
        };

         job_list.sort_by(optimal_order) 
    }

    // Returns the completion times of the JobList. Accidentally implemented this instead
    // of the weighted version. Whoops!
    pub fn completion_times(&self) -> Vec<u32> {
        let JobList(job_list) = self;

        let completion_times = job_list.iter()
            .fold((vec![], 0), |mut acc, x| {
                acc.1 += x.length;
                acc.0.push(acc.1);
                acc
            });
        completion_times.0
    }

    // Returns the weighted completion times of the JobList.
    pub fn weighted_completion_times(&self) -> Vec<u32> {
        let JobList(job_list) = self;

        // Note here that acc.0 is the weighted completion times, while acc.1 is the completion
        // time of the last job.
        let weighted_completion_times = job_list.iter()
            .fold((vec![], 0), |mut acc, x| {
                acc.1 += x.length;
                acc.0.push(acc.1 * x.weight);
                acc
            });
        weighted_completion_times.0
    }

}
