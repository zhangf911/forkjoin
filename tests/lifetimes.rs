extern crate forkjoin;
extern crate rand;

use forkjoin::{TaskResult,ForkPool,AlgoStyle,SummaStyle,Algorithm};

#[test]
fn lifetimes() {
    let forkpool = ForkPool::with_threads(4);
    let sortpool = forkpool.init_algorithm(Algorithm {
        fun: quicksort_task,
        style: AlgoStyle::Summa(SummaStyle::NoArg(quicksort_join)),
    });

    let mut data: Vec<usize> = vec![10,1,8,3,9,2,7,4,6,5];
    {
        let job = sortpool.schedule(&mut data[..]);
        job.recv().unwrap();
    }

    assert_eq!(vec![1,2,3,4,5,6,7,8,9,10], data);
}

#[cfg(test)]
fn quicksort_par<'a>(d: &'a mut[usize], forkpool: ForkPool<&'a mut [usize], ()>) {

}

#[cfg(test)]
fn quicksort_task(d: &mut [usize]) -> TaskResult<&mut [usize], ()> {
    let len = d.len();
    if len <= 1 {
        TaskResult::Done(())
    } else {
        let pivot = partition(d);
        let (low, tmp) = d.split_at_mut(pivot);
        let (_, high) = tmp.split_at_mut(1);

        TaskResult::Fork(vec![low, high], None)
    }
}

#[cfg(test)]
fn quicksort_join(_: &[()]) -> () {}


#[cfg(test)]
fn partition(d: &mut[usize]) -> usize {
    let last = d.len()-1;
    let pi = pick_pivot(d);
    let pv = d[pi];
    d.swap(pi, last); // Put pivot last
    let mut store = 0;
    for i in 0..last {
        if d[i] <= pv {
            d.swap(i, store);
            store += 1;
        }
    }
    if d[store] > pv {
        d.swap(store, last);
        store
    } else {
        last
    }
}

#[cfg(test)]
fn pick_pivot(d: &[usize]) -> usize {
    let len = d.len();
    if len < 3 {
        0
    } else {
        let is = [0, len/2, len-1];
        let mut vs = [d[0], d[len/2], d[len-1]];
        vs.sort();
        for i in is.iter() {
            if d[*i] == vs[1] {
                return *i;
            }
        }
        unreachable!();
    }
}
