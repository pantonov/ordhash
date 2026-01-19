use ordhash::OrdHash;

#[test]
fn new_is_empty() {
    let mut m: OrdHash<i32, i32> = OrdHash::new();
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
    assert_eq!(m.used_entries(), 0);
    assert_eq!(m.peek_front(), None);
    assert_eq!(m.pop_front(), None);
}

#[test]
fn default_is_empty() {
    let m: OrdHash<i32, i32> = OrdHash::default();
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
    assert_eq!(m.used_entries(), 0);
}

#[test]
fn with_capacity_behaves_like_new() {
    let mut m = OrdHash::with_capacity(16);
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
    assert_eq!(m.used_entries(), 0);

    m.push_back(1, "one");
    assert_eq!(m.get(&1), Some(&"one"));
}

#[test]
fn reserve_allows_additional_inserts() {
    let mut m = OrdHash::new();
    m.reserve(8);

    m.push_back(1, "one");
    m.push_back(2, "two");
    assert_eq!(m.len(), 2);
    assert_eq!(m.used_entries(), 2);
}

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
    m.push_back(300, "x");

    assert_eq!(m.mark_unused(&100), Some(&"x"));
    assert_eq!(m.get(&100), None);
    assert_eq!(m.get(&200, ), Some(&"y"));
    assert_eq!(m.len(), 2);
    assert_eq!(m.refresh(&100), Some(&"x"));
    assert_eq!(m.len(), 3);
    while let Some(_) = m.pop_front() {}
    assert_eq!(m.is_empty(), true);
}

#[test]
fn used_entries_tracks_stale_entries() {
    let mut m = OrdHash::new();
    m.push_back(1, "one");
    m.push_back(2, "two");
    m.push_back(1, "one_updated");

    assert_eq!(m.len(), 2);
    assert_eq!(m.used_entries(), 3);

    assert_eq!(m.pop_front(), Some((2, "two"))); 
    assert_eq!(m.len(), 1);
    assert_eq!(m.used_entries(), 1);

    assert_eq!(m.pop_front(), Some((1, "one_updated"))); 
    assert_eq!(m.len(), 0);
    assert_eq!(m.used_entries(), 0);
}

#[test]
fn mark_unused_and_refresh_behaviour() {
    let mut m = OrdHash::new();
    m.push_back(10, "ten");
    m.push_back(20, "twenty");

    assert_eq!(m.mark_unused(&10), Some(&"ten"));
    assert_eq!(m.len(), 1);
    assert_eq!(m.used_entries(), 2);
    assert_eq!(m.get(&10), None);

    assert_eq!(m.refresh(&10), Some(&"ten"));
    assert_eq!(m.len(), 2);
    assert_eq!(m.used_entries(), 3);

    assert_eq!(m.refresh(&30), None);
    assert_eq!(m.len(), 2);
}

#[test]
fn peek_front_skips_unused_entries() {
    let mut m = OrdHash::new();
    m.push_back(1, "one");
    m.push_back(2, "two");
    m.push_back(3, "three");

    assert_eq!(m.mark_unused(&1), Some(&"one"));
    let front = m.peek_front().map(|(k, v)| (*k, *v));
    assert_eq!(front, Some((2, "two")));
}

#[test]
fn get_returns_none_for_missing_or_unused() {
    let mut m = OrdHash::new();
    assert_eq!(m.get(&1), None);

    m.push_back(1, "one");
    assert_eq!(m.get(&1), Some(&"one"));

    let _ = m.mark_unused(&1);
    assert_eq!(m.get(&1), None);
}

#[test]
fn value_overwrite_and_peek_behaviour() {
    let mut m = OrdHash::new();
    m.push_back(1, "one");
    m.push_back(3, "three");
    m.push_back(2, "two");   
    m.push_back(3, "three_updated");
    assert_eq!(m.len(), 3);
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
