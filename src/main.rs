mod chap1;
mod chap2;
mod chap3;

fn main() {
    chap1::run_1_1();
    chap1::run_1_2();
    chap1::run_1_2_1();
    chap1::run_1_2_2();
    chap1::run_1_2_2_1();
    chap1::run_1_2_3();
    chap2::run_2();
    chap2::run_2_1();
    chap2::run_2_2();
    chap2::run_2_3();
    chap3::run_3_1();
    chap3::run_3_2();
    chap3::run_3_2_1();
    chap3::run_3_2_2();

    println!("{}", "hello world");
}
