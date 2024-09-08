use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Read};
use std::time::{Duration, Instant};

mod problem;

#[cfg(test)]
mod tests {
    use super::*; // Import functions from outer scope

    #[test]
    fn run_all_tests_sequentially() {
        trial_tests();
        sample_tests().unwrap();
        complete_tests().unwrap();
    }

    //#[test]
    fn trial_tests() {
        std::env::set_var("RUST_TEST_NOCAPTURE", "1");
        {
            let mut unsorted = vec![1f32, 8f32, -1f32];
            println!("Array:");
            println!("{:?}", unsorted);
            println!("");

            let expected = vec![-1f32, 1f32, 8f32];
            println!("Expected Output:");
            println!("{:?}", expected);

            let ln = unsorted.len();
            problem::giga_sort(&mut unsorted, ln as i32);
            println!("Returned Output:");
            println!("{:?}", unsorted);

            assert_eq!(unsorted, expected);
        }
        println!("test case 1 passed");
        {
            let mut unsorted = vec![1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32];
            println!("Array:");
            println!("{:?}", unsorted);
            println!("");

            let expected = vec![1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32];
            println!("Expected Output:");
            println!("{:?}", expected);

            let ln = unsorted.len();
            problem::giga_sort(&mut unsorted, ln as i32);
            println!("Returned Output:");
            println!("{:?}", unsorted);

            assert_eq!(unsorted, expected);
        }
        println!("test case 2 passed");
        {
            let mut unsorted = vec![-1f32, -2f32, -3f32, -4f32, -5f32, -6f32, -7f32];
            println!("Array:");
            println!("{:?}", unsorted);
            println!("");

            let expected = vec![-7f32, -6f32, -5f32, -4f32, -3f32, -2f32, -1f32];
            println!("Expected Output:");
            println!("{:?}", expected);

            let ln = unsorted.len();
            problem::giga_sort(&mut unsorted, ln as i32);
            println!("Returned Output:");
            println!("{:?}", unsorted);

            assert_eq!(unsorted, expected);
        }
        println!("test case 3 passed");
        {
            let mut unsorted = vec![1f32, 1f32, 1f32, 1f32, 1f32];
            println!("Array:");
            println!("{:?}", unsorted);
            println!("");

            let expected = vec![1f32, 1f32, 1f32, 1f32, 1f32];
            println!("Expected Output:");
            println!("{:?}", expected);

            let ln = unsorted.len();
            problem::giga_sort(&mut unsorted, ln as i32);
            println!("Returned Output:");
            println!("{:?}", unsorted);

            assert_eq!(unsorted, expected);
        }
        println!("test case 4 passed");
        {
            let mut unsorted = vec![2.001, 1.11, 1.01, 1.21, 1.013, 1.012, 1.011];
            println!("Array:");
            println!("{:?}", unsorted);
            println!("");

            let expected = vec![1.01, 1.011, 1.012, 1.013, 1.11, 1.21, 2.001];
            println!("Expected Output:");
            println!("{:?}", expected);

            let ln = unsorted.len();
            problem::giga_sort(&mut unsorted, ln as i32);
            println!("Returned Output:");
            println!("{:?}", unsorted);

            assert_eq!(unsorted, expected);
        }
        println!("test case 5 passed");

        println!("=============================================");
        println!("=============================================");
        println!("=============================================");
    }

    //use std::fs::File;
    use std::io::{self, BufReader, Read};
    use std::time::Instant;

    //#[test]
    fn sample_tests() -> io::Result<()> {
        std::env::set_var("RUST_TEST_NOCAPTURE", "1");

        let file = File::open("tests\\FasterSort_Easy.txt")?;
        let mut reader = BufReader::new(file);

        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        let test_cases = i32::from_le_bytes(bytes);

        let mut max_time = 0;
        let mut avg_time = 0;
        let mut eval = 0;
        for i in 0..test_cases {
            println!("test {} started", i + 1);

            reader.read_exact(&mut bytes)?;
            let array_size = i32::from_le_bytes(bytes);
            let mut arr = Vec::with_capacity(array_size as usize);

            for _j in 0..array_size {
                reader.read_exact(&mut bytes)?;
                let value = f32::from_le_bytes(bytes);
                arr.push(value);
            }

            let mut actual_result = arr.clone();

            let start1 = Instant::now();
            actual_result.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let end1 = Instant::now();
            let elapsed_time1 = end1 - start1;
            println!(
                "Time taken: {} milliseconds using sortBy",
                elapsed_time1.as_millis()
            );

            let start2 = Instant::now();
            problem::giga_sort(&mut arr, array_size);
            let end2 = Instant::now();
            let elapsed_time2 = end2 - start2;
            println!(
                "Time taken: {} milliseconds using quicksort",
                elapsed_time2.as_millis()
            );
            if elapsed_time2.as_millis() > max_time {
                max_time = elapsed_time2.as_millis();
            }
            avg_time += elapsed_time2.as_millis();

            assert_eq!(arr, actual_result);
            eval += 1;
            println!("test {} finished", i + 1);
        }

        println!("FINAL EVALUATION (%) = {}", 10 * eval);
        println!(
            "MAX TIME (ms) = {}, AVG TIME (ms) = {}",
            max_time,
            avg_time / eval
        );
        println!("=============================================");
        println!("=============================================");
        println!("=============================================");
        Ok(())
    }

    //#[test]
    fn complete_tests() -> io::Result<()> {
        std::env::set_var("RUST_TEST_NOCAPTURE", "1");

        let file = File::open("tests\\FasterSort_Hard.txt")?;
        let mut reader = BufReader::new(file);

        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        let test_cases = i32::from_le_bytes(bytes);

        let mut max_time = 0;
        let mut avg_time = 0;
        let mut eval = 0;
        for i in 0..test_cases {
            println!("test {} started", i + 1);

            reader.read_exact(&mut bytes)?;
            let array_size = i32::from_le_bytes(bytes);
            let mut arr = Vec::with_capacity(array_size as usize);

            for _j in 0..array_size {
                reader.read_exact(&mut bytes)?;
                let value = f32::from_le_bytes(bytes);
                arr.push(value);
            }

            let mut actual_result = arr.clone();

            let start1 = Instant::now();
            actual_result.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let end1 = Instant::now();
            let elapsed_time1 = end1 - start1;
            println!(
                "Test Case {}: timeOutInMillisec = {}",
                i + 1,
                elapsed_time1.as_millis()
            );

            let start2 = Instant::now();
            problem::giga_sort(&mut arr, array_size);
            let end2 = Instant::now();
            let elapsed_time2 = end2 - start2;
            println!("Time taken: {}", elapsed_time2.as_millis());
            if elapsed_time2.as_millis() > max_time {
                max_time = elapsed_time2.as_millis();
            }
            avg_time += elapsed_time2.as_millis();

            assert_eq!(arr, actual_result);
            eval += 1;
            println!("test {} finished", i + 1);
        }

        println!("FINAL EVALUATION (%) = {}", 10 * eval);
        println!(
            "MAX TIME (ms) = {}, AVG TIME (ms) = {}",
            max_time,
            avg_time / eval
        );
        println!("=============================================");
        println!("=============================================");
        println!("=============================================");
        Ok(())
    }
}

fn main() {
    println!("hello world!");
}
