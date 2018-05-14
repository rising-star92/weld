//! Tests related to the Vector type.

extern crate weld;
use weld::weld_value_data;

mod common;
use common::*;

#[test]
fn map_comparison() {
    let code = "|e0: vec[i32]| map(e0, |a: i32| a == i32(100))";
    let conf = default_conf();

    let input_vec = [100, 200, 0, 100];
    let ref input_data = WeldVec::from(&input_vec);

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const WeldVec<bool> };
    let result = unsafe { (*data).clone() };
    assert_eq!(result.len as usize, input_vec.len());
    for i in 0..(result.len as isize) {
        assert_eq!(
            unsafe { *result.data.offset(i) },
            input_vec[i as usize] == 100
        )
    }

    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn eq_between_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<i32>,
        y: WeldVec<i32>,
    }
    let conf = default_conf();

    let code = "|e0: vec[i32], e1: vec[i32]| e0 == e1";
    let input_vec1 = [1, 2, 3, 4, 5];
    let input_vec2 = [1, 2, 3, 4, 5];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, true);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn eq_between_diff_length_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<i32>,
        y: WeldVec<i32>,
    }
    let conf = default_conf();

    let code = "|e0: vec[i32], e1: vec[i32]| e0 == e1";
    let input_vec1 = [1, 2, 3, 4, 5];
    let input_vec2 = [1, 2, 3, 4, 5, 6, 7];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, false);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn ne_between_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<i32>,
        y: WeldVec<i32>,
    }
    let conf = default_conf();

    let code = "|e0: vec[i32], e1: vec[i32]| e0 != e1";
    let input_vec1 = [1, 2, 3, 4, 5];
    let input_vec2 = [3, 2, 3, 4, 5];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, true);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn lt_between_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<i32>,
        y: WeldVec<i32>,
    }
    let conf = default_conf();

    let code = "|e0: vec[i32], e1: vec[i32]| e0 < e1";
    let input_vec1 = [1, 2, 3, 4, 5];
    let input_vec2 = [2, 3, 4, 5, 6];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, true);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn le_between_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<i32>,
        y: WeldVec<i32>,
    }
    let conf = default_conf();

    let code = "|e0: vec[i32], e1: vec[i32]| e0 <= e1";
    let input_vec1 = [-1, 0, 3, 4, 5];
    let input_vec2 = [-1, -3, 4, 5, 6];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, false);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn le_between_unsigned_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<i32>,
        y: WeldVec<i32>,
    }
    let conf = default_conf();

    // Note that we pass our integers in as unsigneds in Weld
    let code = "|e0: vec[u32], e1: vec[u32]| e0 <= e1";
    let input_vec1 = [-1, 0, 3, 4, 5];
    let input_vec2 = [-1, -3, 4, 5, 6];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, true);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn eq_between_u8_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<u8>,
        y: WeldVec<u8>,
    }
    let conf = default_conf();

    let code = "|e0: vec[u8], e1: vec[u8]| e0 == e1";
    let input_vec1 = [1u8, 2u8, 3u8, 4u8, 5u8];
    let input_vec2 = [1u8, 2u8, 3u8, 4u8, 5u8];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, true);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn eq_between_different_length_u8_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<u8>,
        y: WeldVec<u8>,
    }
    let conf = default_conf();

    let code = "|e0: vec[u8], e1: vec[u8]| e0 == e1";
    let input_vec1 = [1u8, 2u8, 3u8, 4u8, 5u8];
    let input_vec2 = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, false);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn le_between_u8_vectors() {
    #[allow(dead_code)]
    struct Args {
        x: WeldVec<u8>,
        y: WeldVec<u8>,
    }
    let conf = default_conf();

    let code = "|e0: vec[u8], e1: vec[u8]| e0 <= e1";
    let input_vec1 = [1u8, 2u8, 3u8, 4u8, 5u8];
    let input_vec2 = [1u8, 2u8, 3u8, 255u8, 5u8];
    let ref input_data = Args {
        x: WeldVec::from(&input_vec1),
        y: WeldVec::from(&input_vec2),
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const bool };
    let result = unsafe { *data };
    assert_eq!(result, true);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn simple_vector_lookup() {
    let code = "|x:vec[i32]| lookup(x, 3L)";
    let conf = default_conf();

    let input_vec = [1, 2, 3, 4, 5];
    let ref input_data = WeldVec::from(&input_vec);

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const i32 };
    let result = unsafe { *data };
    assert_eq!(result, input_vec[3]);

    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn simple_vector_slice() {
    let code = "|x:vec[i32]| slice(x, 1L, 3L)";
    let conf = default_conf();

    let input_vec = [1, 2, 3, 4, 5];
    let ref input_data = WeldVec::from(&input_vec);

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const WeldVec<i32> };
    let result = unsafe { (*data).clone() };
    let output = [2, 3, 4];

    assert_eq!(output.len() as isize, result.len as isize);
    for i in 0..(result.len as isize) {
        assert_eq!(unsafe { *result.data.offset(i) }, output[i as usize])
    }

    unsafe { free_value_and_module(ret_value) };

    // Test slicing out of bounds case
    let conf = default_conf();

    let input_vec = [1, 2];
    let ref input_data = WeldVec::from(&input_vec);

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const WeldVec<i32> };
    let result = unsafe { (*data).clone() };
    let output = [2];

    assert_eq!(output.len() as isize, result.len as isize);
    for i in 0..(result.len as isize) {
        assert_eq!(unsafe { *result.data.offset(i) }, output[i as usize])
    }

    unsafe { free_value_and_module(ret_value) };
}
