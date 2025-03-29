struct Span(tracy_client::Span);

// #[derive(Debug)]
// struct SpanStack {
//     _spans: HashMap<SpanId, Span>,
//     id: NonZeroU64,
// }

// impl SpanStack {
//     fn new() -> Self {
//         return SpanStack { _spans: HashMap::new(), id: NonZeroU64::new(1).unwrap() }
//     }

//     fn next_id(&mut self) -> SpanId {
//         // This will only panic if we have 2**64 elements.
//         self.id = self.id.checked_add(1).unwrap();
//         SpanId(self.id)
//     }

//     fn push_span(&mut self, span: tracy_client::Span) -> SpanId {
//         let id = self.next_id();
//         self._spans.insert(id, Span(span));
//         id
//     }
// }

use std::fmt;

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Span")
    }
}

pub struct PerfSpanIter<T, I: Iterator<Item = T>> {
    inner: I,
    _span: Option<Span>,
}

impl<T, I> Iterator for PerfSpanIter<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self._span = Some(Span(tracy_client::span!("perf_span")));
        let val = self.inner.next();
        self._span = None;
        val
    }
}

pub fn make_iter_with_perf_span<T, I: Iterator<Item = T>>(
    inner: I,
) -> PerfSpanIter<T, I> {
    PerfSpanIter { inner, _span: None }
}
