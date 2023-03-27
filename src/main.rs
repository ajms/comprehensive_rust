mod borwein_pi;
mod exercises;
mod gui_library;
mod health_statistics;
mod library;
mod luhn_alg;
mod polygon;
mod webserver_routing;

fn main() {
    borwein_pi::approximate_pi(3);
    exercises::exercise1();
    exercises::exercise2();
    library::test_library();
    exercises::exercise4();
    health_statistics::health_stats();
    polygon::exercise7();
    luhn_alg::exercise8();
    webserver_routing::exercise9();
    gui_library::exercise11();
}
