use crate::calculate;

#[test]
fn rdm_1() {
    let res = calculate("1 + (1 - 5)").unwrap();
    assert_eq!(-3f64, res.0);
}
#[test]
fn rdm_2() {
    let res = calculate("eins plus auf eins minus fünf zu").unwrap();
    assert_eq!(-3f64, res.0);
}
#[test]
fn rdm_3() {
    let res = calculate("eins + auf 1 - fünf)").unwrap();
    assert_eq!(-3f64, res.0);
}
#[test]
fn rdm_4() {
    let res = calculate("eintausendfünfhundertdreiundsiebzig mal (-34 plus 3 mal sechshunderteinundvierzig) durch acht").unwrap();
    assert_eq!(371_424.625f64, res.0);
}
#[test]
fn rdm_5() {
    let res = calculate("eins plus (minus drei mal 6) durch 2 minus (drei mal 4)").unwrap();
    assert_eq!(-20f64, res.0);
}

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