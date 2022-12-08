fn main() {
    let mut vec = vec![2, 3, 4, 1, 14];
    bubble_sort(&mut vec);
    for i in vec {
        println!("{:?}", i)
    }
}

///bubble_sort 方法根据相邻两个元素比较来排序，如果后面的比起那面的大，就进行交换，直到全都交换过为止。所有实现 PartialOrd trait 的都可以排序.
pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    let size = list.len();
    // 空集合不需排序，直接返回
    if size <= 1 {
        return;
    }

    for i in 0..(size - 1) {
        let mut swapped = false;
        for j in 1..(size - i) {
            // 比较相邻的元素 如果第一个比第二个大，就进行交换
            if list[j - 1] > list[j] {
                list.swap(j - 1, j);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![2, 3, 4, 1, 30, 14];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 14, 30]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            "c".to_string(),
            "d".to_string(),
            "a".to_string(),
            "b".to_string(),
        ];
        bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
    }
}
