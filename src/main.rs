mod borwein_pi;
mod exercises;
mod library;

fn main() {
    borwein_pi::approximate_pi(3);
    exercises::exercise1();
    exercises::exercise2();
    library::test_library();
    exercises::exercise4();
}
