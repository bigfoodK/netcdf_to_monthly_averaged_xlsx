use crate::types::DaysOfMonth;

pub fn split_to_worker_count(
    days_of_months: Vec<DaysOfMonth>,
    worker_count: u8,
) -> Vec<Vec<DaysOfMonth>> {
    let mut result: Vec<Vec<DaysOfMonth>> = Vec::new();
    for _ in 0..worker_count {
        result.push(Vec::new());
    }
    for (index, days_of_month) in days_of_months.into_iter().enumerate() {
        let id = index % worker_count as usize;
        let days_of_months_for_worker = result.get_mut(id).unwrap();
        days_of_months_for_worker.push(days_of_month);
    }
    result
}
