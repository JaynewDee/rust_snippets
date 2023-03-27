trait SortStrategy<T> {
    fn sort(&self, data: &mut [T]);
}

struct BubbleSort;
struct QuickSort;

impl<T> SortStrategy<T> for BubbleSort
where
    T: std::cmp::Ord,
{
    fn sort(&self, data: &mut [T]) {
        for i in 0..data.len() {
            for j in 0..data.len() - i - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
}

impl<T> SortStrategy<T> for QuickSort
where
    T: std::cmp::Ord,
{
    fn sort(&self, data: &mut [T]) {
        if data.len() > 1 {
            let pivot = partition(data);
            self.sort(&mut data[..pivot]);
            self.sort(&mut data[pivot + 1..]);
        }
    }
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let pivot = data.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if data[j] < data[pivot] {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, pivot);
    i
}

struct Sorter<T> {
    strategy: Box<dyn SortStrategy<T>>,
}

impl<T> Sorter<T> {
    fn new(strategy: Box<dyn SortStrategy<T>>) -> Self {
        Sorter { strategy }
    }

    fn sort(&self, data: &mut [T]) {
        self.strategy.sort(data);
    }
}

fn main() {
    let data = &mut [3, 2, 1, 4, 5];
    let bubble_sort = Box::new(BubbleSort);
    let quick_sort = Box::new(QuickSort);
    let sorter = Sorter::new(bubble_sort);
    sorter.sort(data);
    println!("{:?}", data);
    let sorter = Sorter::new(quick_sort);
    sorter.sort(data);
    println!("{:?}", data);
}
