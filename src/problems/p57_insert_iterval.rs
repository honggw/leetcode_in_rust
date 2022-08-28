
/**
You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
Return intervals after the insertion.
 */

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {

    use std::cmp::Ordering;

    #[derive(PartialEq,Eq,Clone)]
    struct Interval {
        start:i32,
        end:i32,
    }

    impl Ord for Interval {
        fn cmp(&self,other:&Self) -> Ordering {
            match self.start.cmp(&other.start) {
                Less => {
                    Less
                },
                Equal => {
                    self.end.cmp(&other.end)
                },
                Greater => {
                    Greater
                }
            }
        }
    }

    impl PartialOrd for Interval {
        fn partial_cmp(&self,other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Interval {

        fn new(s:i32, e: i32) -> Self {
            Self {
                start:s,
                end:e,
            }
        }

        fn swap(&mut self,other:&mut Self) {
            let s = other.start;
            let e = other.end;
            other.start = self.start;
            other.end = self.end;
            self.start = s;
            self.end = e;
        }

        fn overlap(&self,other:&Self) -> bool {
            if self.start <= other.end && self.end >= other.start {
                true
            } else {
                false
            }
        }

        fn merge(&mut self, other: &Self) -> bool {
            if self.overlap(other) {
                self.start = if self.start > other.start { other.start } else { self.start };
                self.end = if self.end > other.end { self.end } else { other.end };
                true
            } else {
                false
            }
        }

        fn data(&self)->Vec<i32> {
            vec![self.start,self.end]
        }

    }

    let mut result:Vec<Vec<i32>> = Vec::new();

    let mut interval_list = intervals.iter()
        .map(|v| Interval::new(*v.get(0).unwrap(), *v.get(1).unwrap()) )
        .collect::<Vec<Interval>>();

    interval_list.sort();
    let mut inserted = false;
    let mut new_interval_obj = Interval::new(*new_interval.get(0).unwrap(), *new_interval.get(1).unwrap());

    let last = interval_list.iter_mut().reduce(|accume,next| {

        if !inserted {
            if accume.merge(&mut new_interval_obj) {
                inserted = true;
            } else if accume > &mut new_interval_obj {
                result.push(new_interval_obj.data());
                inserted = true;
            }
        }

        if accume.merge(next) {
            accume
        } else {
            result.push(accume.data());
            next
        }

    });

    if let Some(last_interval) = last {
        if !inserted {
            if last_interval.merge(&mut new_interval_obj) {
                result.push(last_interval.data());
            } else {
                if last_interval > &mut new_interval_obj {
                    result.push(new_interval_obj.data());
                    result.push(last_interval.data());
                } else {
                    result.push(last_interval.data());
                    result.push(new_interval_obj.data());
                }
            }
        } else {
            result.push(last_interval.data());
        }
    } else {
        if !inserted {
            result.push(new_interval_obj.data());
        }
    }


    result
}

#[test]
fn test() {
    println!("result={:?}",insert(vec![vec![9,10],vec![3,5],vec![1,2],vec![2,3]],vec![1,10]));
    println!("result={:?}",insert(vec![vec![9,10],vec![3,5],vec![1,2],vec![2,3]],vec![5,10]));
    println!("result={:?}",insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]],vec![4,10]));
    println!("result={:?}",insert(vec![],vec![4,10]));
    println!("result={:?}",insert(vec![vec![1,5]],vec![6,8]));
    println!("result={:?}",insert(vec![vec![3,5],vec![6,8],vec![10,11]],vec![1,3]));
    println!("result={:?}",insert(vec![vec![3,5],vec![6,8],vec![10,11]],vec![1,10]));
    println!("result={:?}",insert(vec![vec![3,5],vec![6,8],vec![10,11]],vec![1,2]));
    println!("result={:?}",insert(vec![vec![3,5],vec![6,8],vec![10,11]],vec![12,13]));
}
