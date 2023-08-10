pub trait ValueStream {
    type Item;

    fn next_value(&mut self) -> Option<Self::Item>;
}

#[derive(Clone, Debug, PartialEq, PartialOrd, )]
pub(crate) struct Mono<V> {
    _value: Option<V>,
}

impl<V> ValueStream for Mono<V> {
    type Item = V;

    fn next_value(&mut self) -> Option<Self::Item> {
        self._value.take()
    }
}