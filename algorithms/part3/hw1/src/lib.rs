pub mod scheduling;

#[cfg(test)]
mod tests {
    use super::scheduling::*;

    fn job_list_with_ties() -> JobList {
        JobList(vec![
            Job { weight: 1, length: 2 },
            Job { weight: 3, length: 4 },
            Job { weight: 5, length: 6 }])
    }

    fn job_list_no_ties() -> JobList {
        JobList(vec![
            Job { weight: 0, length: 2 },
            Job { weight: 3, length: 4 },
            Job { weight: 6, length: 6 }])
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
            Job { weight: 5, length: 6 },
            Job { weight: 3, length: 4 },
            Job { weight: 1, length: 2 }]);
        let mut test_job_list = job_list_with_ties();
        test_job_list.schedule_jobs_additive();
        assert_eq!(sorted_job_list, test_job_list)
    }

    #[test]
    fn test_additive_scheduler_no_ties() {
        let sorted_job_list = JobList(vec![
            Job { weight: 6, length: 6 },
            Job { weight: 3, length: 4 },
            Job { weight: 0, length: 2 }]);
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
    fn test_optimal_scheduler () {
        let mut job_list = job_list_with_ties();
        let sorted_job_list = JobList(vec![
            Job { weight: 5, length: 6 },
            Job { weight: 3, length: 4 },
            Job { weight: 1, length: 2 }
        ]);
        job_list.schedule_jobs_optimal();
        assert_eq!(job_list, sorted_job_list);

        let mut job_list = job_list_no_ties();
        let sorted_job_list = JobList(vec![
            Job { weight: 6, length: 6 },
            Job { weight: 3, length: 4 },
            Job { weight: 0, length: 2 }
        ]);
        job_list.schedule_jobs_optimal();
        assert_eq!(job_list, sorted_job_list);
    }

}