use crate::RifError;

#[derive(Debug, PartialEq)]
pub enum Tracer {
    /// Function call tracer to trace all kernel functions.
    Function,
    /// Similar to the function tracer except that the
    /// function tracer probes the functions on their entry
    /// whereas the function graph tracer traces on both entry
    /// and exit of the functions. It then provides the ability
    /// to draw a graph of function calls similar to C code
    /// source.
    FunctionGraph,
    /// The block tracer. The tracer used by the blktrace user
    /// application.
    Block,
    /// The Hardware Latency tracer is used to detect if the hardware
    /// produces any latency. See "Hardware Latency Detector" section
    /// below.
    HardwareLatency,
    /// Traces the areas that disable interrupts and saves
    /// the trace with the longest max latency.
    /// See tracing_max_latency. When a new max is recorded,
    /// it replaces the old trace. It is best to view this
    /// trace with the latency-format option enabled, which
    /// happens automatically when the tracer is selected.
    IRQsOff,
    /// Similar to irqsoff but traces and records the amount of
    /// time for which preemption is disabled.
    PreemptOff,
    /// Similar to irqsoff and preemptoff, but traces and
    /// records the largest time for which irqs and/or preemption
    /// is disabled.
    PreemptIRQsOff,
    /// Traces and records the max latency that it takes for
    /// the highest priority task to get scheduled after
    /// it has been woken up.
    /// Traces all tasks as an average developer would expect.
    Wakeup,
    /// Traces and records the max latency that it takes for just
    /// RT tasks (as the current "wakeup" does). This is useful
    /// for those interested in wake up timings of RT tasks.
    WakeupRealtime,
    /// Traces and records the max latency that it takes for
    /// a SCHED_DEADLINE task to be woken (as the "wakeup" and
    /// "wakeup_rt" does).
    WakeupDeadline,
    /// A special tracer that is used to trace binary module.
    /// It will trace all the calls that a module makes to the
    /// hardware. Everything it writes and reads from the I/O
    /// as well.
    MMIOTrace,
    /// This tracer can be configured when tracing likely/unlikely
    /// calls within the kernel. It will trace when a likely and
    /// unlikely branch is hit and if it was correct in its prediction
    /// of being correct.
    Branch,
    /// This is the "trace nothing" tracer. To remove all
    /// tracers from tracing simply echo "nop" into
    /// current_tracer.
    Nop,
}

impl std::fmt::Display for Tracer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tracer::Function => write!(f, "function"),
            Tracer::FunctionGraph => write!(f, "function_graph"),
            Tracer::Block => write!(f, "blk"),
            Tracer::HardwareLatency => write!(f, "hwlat"),
            Tracer::IRQsOff => write!(f, "irqsoff"),
            Tracer::PreemptOff => write!(f, "preemptoff"),
            Tracer::PreemptIRQsOff => write!(f, "preemptirqsoff"),
            Tracer::Wakeup => write!(f, "wakeup"),
            Tracer::WakeupRealtime => write!(f, "wakeup_rt"),
            Tracer::WakeupDeadline => write!(f, "wakeup_dl"),
            Tracer::MMIOTrace => write!(f, "mmiotrace"),
            Tracer::Branch => write!(f, "branch"),
            Tracer::Nop => write!(f, "nop"),
        }
    }
}

impl std::str::FromStr for Tracer {
    type Err = RifError;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        let tracer = match item.trim() {
            "function" => Tracer::Function,
            "function_graph" => Tracer::FunctionGraph,
            "blk" => Tracer::Block,
            "hwlat" => Tracer::HardwareLatency,
            "irqsoff" => Tracer::IRQsOff,
            "preemptoff" => Tracer::PreemptOff,
            "preemptirqsoff" => Tracer::PreemptIRQsOff,
            "wakeup" => Tracer::Wakeup,
            "wakeup_rt" => Tracer::WakeupRealtime,
            "wakeup_dl" => Tracer::WakeupDeadline,
            "mmiotrace" => Tracer::MMIOTrace,
            "branch" => Tracer::Branch,
            "nop" => Tracer::Nop,
            _ => return Err(RifError::InvalidTracer),
        };

        Ok(tracer)
    }
}

#[derive(Debug, PartialEq)]
pub enum TracingStat {
    Off,
    On,
}
