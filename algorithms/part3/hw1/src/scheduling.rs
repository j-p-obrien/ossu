use std::{cmp::Ordering, fs, str::FromStr};

// Struct that holds the weight and length of the job to be scheduled.
#[derive(Debug, Eq, PartialEq)]
pub struct Job {
    pub weight: u64,
    pub length: u64,
}

#[derive(Debug)]
pub struct ParseJobError;

impl Job {
    // Computes weight - length
    pub fn additive_priority(&self) -> i64 {
        self.weight as i64 - self.length as i64
    }

    // computes weight/length
    pub fn multiplicative_priority(&self) -> f64 {
        self.weight as f64 / self.length as f64
    }
}

// Implement FromStr so we can turn a line from the file into a Job.
impl FromStr for Job {
    type Err = ParseJobError;

    fn from_str(line: &str) -> Result<Job, ParseJobError> {
        let job_data: Vec<Result<u64, _>> =
            line.split_whitespace().map(|num| num.parse()).collect();

        if let [Ok(weight), Ok(length)] = job_data[..] {
            return Ok(Job { weight, length });
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
        let job_file = fs::read_to_string(path).expect("Couldn't read file");

        let mut line_iter = job_file.lines();
        // skip first iteration; it contains only the number of jobs.
        line_iter.next();
        let job_list: Vec<Job> = line_iter.map(|line| Job::from_str(line).unwrap()).collect();

        return JobList(job_list);
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
                return j.weight.cmp(&i.weight);
            } else {
                return j_priority.cmp(&i_priority);
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
                return order;
            } else {
                panic!("Prob some kind of division by 0 error.")
            }
        };

        job_list.sort_by(optimal_order)
    }

    // Returns the completion times of the JobList. Accidentally implemented this instead
    // of the weighted version. Whoops!
    pub fn completion_times(&self) -> Vec<u64> {
        let JobList(job_list) = self;

        let completion_times = job_list.iter().fold((vec![], 0), |mut acc, x| {
            acc.1 += x.length;
            acc.0.push(acc.1);
            acc
        });
        completion_times.0
    }

    // Returns the weighted completion times of the JobList.
    pub fn weighted_completion_times(&self) -> Vec<u64> {
        let JobList(job_list) = self;

        // Note here that acc.0 is the weighted completion times, while acc.1 is the completion
        // time of the last job.
        let weighted_completion_times = job_list.iter().fold((vec![], 0), |mut acc, x| {
            acc.1 += x.length;
            acc.0.push(acc.1 * x.weight);
            acc
        });
        weighted_completion_times.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job_list_with_ties() -> JobList {
        JobList(vec![
            Job {
                weight: 1,
                length: 2,
            },
            Job {
                weight: 3,
                length: 4,
            },
            Job {
                weight: 5,
                length: 6,
            },
        ])
    }

    fn job_list_no_ties() -> JobList {
        JobList(vec![
            Job {
                weight: 0,
                length: 2,
            },
            Job {
                weight: 3,
                length: 4,
            },
            Job {
                weight: 6,
                length: 6,
            },
        ])
    }

    #[test]
    fn test_parser() {
        let jobs_from_file = JobList::parse_job_list("testfile1.txt");
        let job_list = job_list_with_ties();
        assert_eq!(job_list, jobs_from_file)
    }

    #[test]
    fn test_additive_scheduler_ties() {
        let sorted_job_list = JobList(vec![
            Job {
                weight: 5,
                length: 6,
            },
            Job {
                weight: 3,
                length: 4,
            },
            Job {
                weight: 1,
                length: 2,
            },
        ]);
        let mut test_job_list = job_list_with_ties();
        test_job_list.schedule_jobs_additive();
        assert_eq!(sorted_job_list, test_job_list)
    }

    #[test]
    fn test_additive_scheduler_no_ties() {
        let sorted_job_list = JobList(vec![
            Job {
                weight: 6,
                length: 6,
            },
            Job {
                weight: 3,
                length: 4,
            },
            Job {
                weight: 0,
                length: 2,
            },
        ]);
        let mut test_job_list = job_list_no_ties();
        test_job_list.schedule_jobs_additive();
        assert_eq!(sorted_job_list, test_job_list)
    }

    #[test]
    fn test_weighted_completion_times() {
        let job_list = job_list_with_ties();
        let completions = vec![2, 18, 60];
        assert_eq!(job_list.weighted_completion_times(), completions);

        let job_list = job_list_no_ties();
        let no_ties_completions = vec![0, 18, 72];
        assert_eq!(job_list.weighted_completion_times(), no_ties_completions);
    }

    #[test]
    fn test_optimal_scheduler() {
        let mut job_list = job_list_with_ties();
        let sorted_job_list = JobList(vec![
            Job {
                weight: 5,
                length: 6,
            },
            Job {
                weight: 3,
                length: 4,
            },
            Job {
                weight: 1,
                length: 2,
            },
        ]);
        job_list.schedule_jobs_optimal();
        assert_eq!(job_list, sorted_job_list);

        let mut job_list = job_list_no_ties();
        let sorted_job_list = JobList(vec![
            Job {
                weight: 6,
                length: 6,
            },
            Job {
                weight: 3,
                length: 4,
            },
            Job {
                weight: 0,
                length: 2,
            },
        ]);
        job_list.schedule_jobs_optimal();
        assert_eq!(job_list, sorted_job_list);
    }
}
