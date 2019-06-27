struct Bucket<K, V> {
    items: Vec<(K, V)>,
}
pub struct HashMap {
    buckets: Vec<Bucket<K, V>>,
}
