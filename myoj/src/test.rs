#[cfg(test)]



mod test {
    use std::sync::MutexGuard;
    use crate::global::Contest;
    use crate::global::CONTEST_INFO;
    use crate::contest;
    #[test]
    pub fn load_contests()
    {
        // contest::get_contest();
        let contest_lock: MutexGuard<Vec<Contest>> = CONTEST_INFO.lock().unwrap();
        let response: Vec<Contest> = (*contest_lock).clone();
        assert_eq!(2,response.len());

    }

}

