#[macro_use]
extern crate criterion;
extern crate iridium;

use criterion::Criterion;
use iridium::assembler::{PIE_HEADER_LENGTH, PIE_HEADER_PREFIX};
use iridium::vm::VM;

mod arithmetic {
    use super::*;

    fn execute_add(c: &mut Criterion) {
        let clos = {
            let mut test_vm = get_test_vm();
            test_vm.program = vec![1, 0, 1, 2];
            test_vm.run_once();
        };

        c.bench_function("execute_add", move |b| b.iter(|| clos));
    }
}

#[test]
fn test_add_opcode() {
    let mut test_vm = get_test_vm();
    test_vm.program = vec![1, 0, 1, 2];
    test_vm.program = prepend_header(test_vm.program);
    test_vm.run();
    assert_eq!(test_vm.registers[2], 15);
}
