use riftrace::{self, Tracer, TracingStat};

#[test]
fn should_set_current_tracer() {
    riftrace::set_current_tracer(Tracer::Function).unwrap();
    assert_eq!(riftrace::get_current_tracer().unwrap(), Tracer::Function);

    riftrace::set_current_tracer(Tracer::FunctionGraph).unwrap();
    assert_eq!(
        riftrace::get_current_tracer().unwrap(),
        Tracer::FunctionGraph
    );

    riftrace::cleanup_tracing().unwrap();
}

#[test]
fn should_enable_disable_tracer() {
    riftrace::set_tracing_on(TracingStat::On).unwrap();
    assert_eq!(riftrace::is_tracing_on().unwrap(), true);

    riftrace::set_tracing_on(TracingStat::Off).unwrap();
    assert_eq!(riftrace::is_tracing_on().unwrap(), false);

    riftrace::cleanup_tracing().unwrap();
}
