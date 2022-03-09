use crate::calculate;

#[test]
fn t1() {
    let res = calculate("1").unwrap();
    assert_eq!(1f64, res.0);
}
#[test]
fn t2() {
    let res = calculate("-1").unwrap();
    assert_eq!(-1f64, res.0);
}
#[test]
fn t3() {
    let res = calculate("(1)").unwrap();
    assert_eq!(1f64, res.0);
}
#[test]
fn t4() {
    let res = calculate("(-1)").unwrap();
    assert_eq!(-1f64, res.0);
}
#[test]
fn t5() {
    let res = calculate("((1))").unwrap();
    assert_eq!(1f64, res.0);
}
#[test]
fn t6() {
    let res = calculate("((-1))").unwrap();
    assert_eq!(-1f64, res.0);
}
#[test]
fn t7() {
    let res = calculate("-((-1))").unwrap();
    assert_eq!(1f64, res.0);
}
#[test]
fn t8() {
    let res = calculate("-((-1 plus 3))").unwrap();
    assert_eq!(-2f64, res.0);
}