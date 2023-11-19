use riftrace::{self, Tracer, TracingControl, TracingStat};

#[test]
fn should_set_current_tracer() {
    let trace_ctrl = TracingControl::new();
    trace_ctrl.set_current_tracer(Tracer::Function).unwrap();
    assert_eq!(trace_ctrl.get_current_tracer().unwrap(), Tracer::Function);

    trace_ctrl
        .set_current_tracer(Tracer::FunctionGraph)
        .unwrap();
    assert_eq!(
        trace_ctrl.get_current_tracer().unwrap(),
        Tracer::FunctionGraph
    );

    trace_ctrl.cleanup_tracing().unwrap();
}

#[test]
fn should_enable_disable_tracer() {
    let trace_ctrl = TracingControl::new();
    trace_ctrl.set_tracing_on(TracingStat::On).unwrap();
    assert!(trace_ctrl.is_tracing_on().unwrap());

    trace_ctrl.set_tracing_on(TracingStat::Off).unwrap();
    assert!(!trace_ctrl.is_tracing_on().unwrap());

    trace_ctrl.cleanup_tracing().unwrap();
}

#[test]
fn should_create_instance() {
    let trace_ctrl = TracingControl::from("bar").unwrap();
    assert!(trace_ctrl
        .get_tracefs_path()
        .unwrap()
        .ends_with("instances/bar"));
}
