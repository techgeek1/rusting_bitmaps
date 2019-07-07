#![cfg(test)]

use crate::container::Container;

/// An internal trait for automating test setup
pub trait TestUtils {
    fn create() -> Self;

    fn fill(&mut self, data: &[u16]);
}

/// Create an array container from the given data set
pub fn make_container<T: TestUtils>(data: &[u16]) -> T {
    let mut container = T::create();
    container.fill(data);

    container
}

/// Run a test using the provided executor function and compare it against the expected value
pub fn run_test<T, U, F>(in_a: &[u16], in_b: &[u16], expected: &[u16], f: F) 
    where T: TestUtils,
          U: TestUtils,
          F: Fn(T, U) -> Container 
{
    let a = make_container::<T>(in_a);
    let b = make_container::<U>(in_b);
    let result = (f)(a, b);

    // Check that the cardinality matches the precomputed result
    let len0 = expected.len();
    let len1 = result.cardinality();
    assert_eq!(
        len0, 
        len1, 
        "\nUnequal cardinality; expected {}, found {}.\n", 
        len0, 
        len1
    );

    // Check that the output matches the precomputed result
    let pass = result.iter()
        .zip(expected.iter());
    
    for (found, expected) in pass {
        assert_eq!(found, *expected, "Sets are not equivalent. Found {}, expected {}", found, *expected);
    }
}

pub const INPUT_A: [u16; 97] = [
    12129, 12375, 13182, 13954, 14628, 14706, 14738, 15585, 17122, 17750,
    18396, 18421, 18846, 19076, 19754, 20145, 20366, 21452, 21867, 24421,
    25247, 25803, 28281, 29337, 29480, 29631, 29734, 29738, 30828, 30926,
    30989, 31801, 32270, 33388, 34083, 34282, 35418, 36121, 36283, 36886,
    36988, 37066, 37714, 37727, 37896, 38530, 39197, 40559, 40730, 41299,
    41320, 42620, 42631, 42706, 47759, 49015, 50432, 50892, 51755, 51777,
    53679, 53863, 54458, 56153, 56828, 57023, 57122, 57255, 57413, 57785,
    58366, 59382, 60813, 61117, 61136, 61752, 61896, 62058, 62213, 62264,
    62302, 62376, 62428, 62673, 62693, 63305, 63308, 63610, 63676, 63783,
    63902, 64232, 64596, 65046, 65320, 65474, 65532
];

pub const INPUT_B: [u16; 100] = [
    443, 557, 1008, 2945, 3316, 3915, 4666, 5386, 5707, 6007,
    6680, 6959, 8048, 8476, 8527, 9316, 9619, 9847, 10340, 11302,
    12129, 12375, 13182, 13954, 14628, 14706, 14738, 15585, 17122, 17750,
    18396, 18421, 18846, 19076, 19754, 20145, 20366, 21452, 21867, 24421,
    25247, 25803, 28281, 29337, 29480, 29631, 29734, 29738, 30926, 30989,
    31801, 34083, 36886, 36988, 37066, 37714, 38530, 39197, 40559, 40730,
    41299, 41320, 42620, 42631, 42706, 42800, 43682, 43718, 44121, 44326,
    44615, 45761, 45824, 46940, 47759, 49015, 50432, 50892, 51755, 51777,
    53679, 53863, 54458, 56153, 56828, 57255, 61752, 62213, 62264, 62376,
    62428, 62673, 62693, 63305, 63610, 63783, 65046, 65320, 65474, 65532
];

pub const RESULT_OR: [u16; 126] = [
    443, 557, 1008, 2945, 3316, 3915, 4666, 5386, 5707, 6007,
    6680, 6959, 8048, 8476, 8527, 9316, 9619, 9847, 10340, 11302,
    12129, 12375, 13182, 13954, 14628, 14706, 14738, 15585, 17122, 17750,
    18396, 18421, 18846, 19076, 19754, 20145, 20366, 21452, 21867, 24421,
    25247, 25803, 28281, 29337, 29480, 29631, 29734, 29738, 30828, 30926,
    30989, 31801, 32270, 33388, 34083, 34282, 35418, 36121, 36283, 36886,
    36988, 37066, 37714, 37727, 37896, 38530, 39197, 40559, 40730, 41299,
    41320, 42620, 42631, 42706, 42800, 43682, 43718, 44121, 44326, 44615,
    45761, 45824, 46940, 47759, 49015, 50432, 50892, 51755, 51777, 53679,
    53863, 54458, 56153, 56828, 57023, 57122, 57255, 57413, 57785, 58366,
    59382, 60813, 61117, 61136, 61752, 61896, 62058, 62213, 62264, 62302,
    62376, 62428, 62673, 62693, 63305, 63308, 63610, 63676, 63783, 63902,
    64232, 64596, 65046, 65320, 65474, 65532
];

pub const RESULT_AND: [u16; 71] = [
    12129, 12375, 13182, 13954, 14628, 14706, 14738, 15585, 17122, 17750,
    18396, 18421, 18846, 19076, 19754, 20145, 20366, 21452, 21867, 24421,
    25247, 25803, 28281, 29337, 29480, 29631, 29734, 29738, 30926, 30989,
    31801, 34083, 36886, 36988, 37066, 37714, 38530, 39197, 40559, 40730,
    41299, 41320, 42620, 42631, 42706, 47759, 49015, 50432, 50892, 51755,
    51777, 53679, 53863, 54458, 56153, 56828, 57255, 61752, 62213, 62264,
    62376, 62428, 62673, 62693, 63305, 63610, 63783, 65046, 65320, 65474,
    65532
];

pub const RESULT_AND_NOT: [u16; 26] = [
    30828, 32270, 33388, 34282, 35418, 36121, 36283, 37727, 37896, 57023,
    57122, 57413, 57785, 58366, 59382, 60813, 61117, 61136, 61896, 62058,
    62302, 63308, 63676, 63902, 64232, 64596
];

pub const RESULT_XOR: [u16; 55] = [
    443, 557, 1008, 2945, 3316, 3915, 4666, 5386, 5707, 6007,
    6680, 6959, 8048, 8476, 8527, 9316, 9619, 9847, 10340, 11302,
    30828, 32270, 33388, 34282, 35418, 36121, 36283, 37727, 37896, 42800,
    43682, 43718, 44121, 44326, 44615, 45761, 45824, 46940, 57023, 57122,
    57413, 57785, 58366, 59382, 60813, 61117, 61136, 61896, 62058, 62302,
    63308, 63676, 63902, 64232, 64596
];

pub const SUBSET_A: [u16; 16] = [
    45761, 45824, 46940, 47759, 49015, 50432, 50892, 51755, 51777,
    53679, 53863, 54458, 56153, 56828, 57255, 61752
];

pub const SUBSET_B: [u16; 30] = [
    44615, 45761, 45824, 46940, 47759, 49015, 50432, 50892, 51755, 51777,
    53679, 53863, 54458, 56153, 56828, 57255, 61752, 62213, 62264, 62376,
    62428, 62673, 62693, 63305, 63610, 63783, 65046, 65320, 65474, 65532
];

pub const RUNS: [u16; 20] = [
    1, 2, 3, 4, 6, 7, 8, 9, 21, 22,
    23, 62, 63, 64, 65, 66, 801, 802, 803, 804
];

pub const NUM_RUNS: usize = 5;