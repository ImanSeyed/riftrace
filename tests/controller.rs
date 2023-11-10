use riftrace::{self, Controller, Tracer, TracingStat};

#[test]
fn should_set_current_tracer() {
    let controller = Controller::new();
    controller.set_current_tracer(Tracer::Function).unwrap();
    assert_eq!(controller.get_current_tracer().unwrap(), Tracer::Function);

    controller
        .set_current_tracer(Tracer::FunctionGraph)
        .unwrap();
    assert_eq!(
        controller.get_current_tracer().unwrap(),
        Tracer::FunctionGraph
    );

    controller.cleanup_tracing().unwrap();
}

#[test]
fn should_enable_disable_tracer() {
    let controller = Controller::new();
    controller.set_tracing_on(TracingStat::On).unwrap();
    assert_eq!(controller.is_tracing_on().unwrap(), true);

    controller.set_tracing_on(TracingStat::Off).unwrap();
    assert_eq!(controller.is_tracing_on().unwrap(), false);

    controller.cleanup_tracing().unwrap();
}
