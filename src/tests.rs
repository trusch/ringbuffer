#[cfg(test)]
use super::*;

#[test]
fn test_if_old_values_are_dropped() {
    let mut buf = RingBuffer::new(4);
    for i in 0..5 {
        buf.push(i);
    }
    assert_eq!(buf.check_if_any(|x| *x == 0), false);
    assert_eq!(buf.check_if_any(|x| *x == 1), true);
    assert_eq!(buf.check_if_any(|x| *x == 2), true);
    assert_eq!(buf.check_if_any(|x| *x == 3), true);
    assert_eq!(buf.check_if_any(|x| *x == 4), true);
}

#[test]
fn test_for_each_can_mutate() {
    let mut buf = RingBuffer::new(4);
    for _ in 0..4 {
        buf.push(1);
    }
    let mut sum = 0;
    buf.for_each(|x| {
        sum += x;
        Ok(())
    })
    .expect("failed to execute");
    assert_eq!(sum, 4);
}

#[test]
fn test_for_each_breaks_on_error() {
    let mut buf = RingBuffer::new(4);
    for i in 0..4 {
        buf.push(i);
    }
    let mut sum = 0;
    let res = buf.for_each(|x| {
        if *x == 2 {
            simple_error::bail!("error");
        }
        sum += x;
        Ok(())
    });
    assert_eq!(res.is_err(), true);
    assert_eq!(res.err().unwrap().description(), "error");
    assert_eq!(sum, 1);
}

#[test]
fn test_contains() {
    let mut buf = RingBuffer::new(1);
    buf.push(1);
    assert_eq!(buf.contains(&1), true);
    assert_eq!(buf.contains(&2), false);
}
