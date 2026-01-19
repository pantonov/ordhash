use ordhash::OrdHash;

#[test]
fn values_order() {
    let mut m = OrdHash::new();
    m.push_back(1, "one");
    m.push_back(3, "three");
    m.push_back(2, "two");

    assert_eq!(m.len(), 3);
    assert_eq!(m.get(&1).copied(), Some("one"));
    assert_eq!(m.get(&2).copied(), Some("two"));
    assert_eq!(m.get(&3).copied(), Some("three"));

    let values = vec!["one", "three", "two"];
    let mut index = 0;
    while let Some((_k, v)) = m.pop_front() {
        assert_eq!(v, values[index as usize]);
        index += 1;
    }
    assert!(m.is_empty());
}

#[test]
fn remove_and_clear_behavior() {
    let mut m = OrdHash::new();
    m.push_back(200, "y");
    m.push_back(100, "x");

    assert_eq!(m.remove(&100), Some("x"));
    assert_eq!(m.get(&100), None);
    assert_eq!(m.get(&200, ), Some(&"y"));
    assert_eq!(m.len(), 1);
}

#[test]
fn value_overwrite_and_peek_behaviour() {
    let mut m = OrdHash::new();
    m.push_back(1, "one");
    m.push_back(3, "three");
    m.push_back(2, "two");   
    m.push_back(3, "three_updated");
    let values = vec!["one", "two", "three_updated"];
    let mut index = 0;
    while let Some((_k, v)) = m.pop_front() {
        assert_eq!(v, values[index as usize]);
        if let Some(pv) = m.peek_front() {
            assert_eq!(*pv.1, values[index + 1 as usize]);
        }
        index += 1;
    }
}
