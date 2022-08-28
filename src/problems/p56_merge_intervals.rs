
/**
 * Given an array of intervals where intervals[i] = [starti, endi], 
 * merge all overlapping intervals, 
 * and return an array of the non-overlapping intervals that cover all the intervals in the input.
 */

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    use std::cmp::Ordering;
    let mut result:Vec<Vec<i32>> = Vec::new();

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

        fn new(s:i32,e:i32) -> Self {
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

        fn merge(&mut self,other:&Self) -> bool{
            if self.overlap(other) {
                self.start = if self.start> other.start { other.start} else {self.start};
                self.end = if self.end > other.end {self.end} else {other.end};
                true
            } else {
                false
            }
        }

        fn data(&self)->Vec<i32> {
            vec![self.start,self.end]
        }

    }

    let mut interval_list = intervals.iter()
        .map( |v| {Interval::new(*v.get(0).unwrap(),*v.get(1).unwrap())})
        .collect::<Vec<Interval>>();

    interval_list.sort();
    let ret = interval_list.iter_mut().reduce(|accum,item| {
        if accum.merge(item) {
            accum
        } else {
            result.push(accum.data());
            item
        }
    });

    if let Some(acc) = ret {
        result.push(acc.data())
    }

    result

}

#[test]
fn test() {
    println!("result={:?}",merge(vec![vec![9,10],vec![3,5],vec![1,2],vec![2,3]]));

}
