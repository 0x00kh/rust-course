use job1::test1;
use job2_1::test2_1;
use job2_2::test2_2;

mod job1;
mod job2_1;
mod job2_2;

fn main() {
    test1();
    test2_1();
    test2_2();
}
