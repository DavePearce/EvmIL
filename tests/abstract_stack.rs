use evmil::analysis::{AbstractStack,AbstractValue,EMPTY_STACK};
use evmil::util::u256;

const ZERO : AbstractValue = AbstractValue::Known(u256::from_u64(0));
const ONE : AbstractValue = AbstractValue::Known(u256::from_u64(1));
const TWO : AbstractValue = AbstractValue::Known(u256::from_u64(2));
const THREE : AbstractValue = AbstractValue::Known(u256::from_u64(3));
const UNKNOWN : AbstractValue = AbstractValue::Unknown;

#[test]
fn test_abstract_stack_01() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.values(),vec![]);
    assert_eq!(st.push(UNKNOWN), AbstractStack::new(1..1,vec![]));
}

#[test]
fn test_abstract_stack_02() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.push(UNKNOWN).push(UNKNOWN), AbstractStack::new(2..2,vec![]));
}

#[test]
fn test_abstract_stack_03() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.push(UNKNOWN).push(UNKNOWN).pop(), AbstractStack::new(1..1,vec![]));
}

#[test]
fn test_abstract_stack_04() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.push(UNKNOWN).push(ZERO), AbstractStack::new(1..1,vec![ZERO]));
}

#[test]
fn test_abstract_stack_05() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE), AbstractStack::new(1..1,vec![ZERO,ONE]));
}

#[test]
fn test_abstract_stack_06() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE).pop(), AbstractStack::new(1..1,vec![ZERO]));
}

#[test]
fn test_abstract_stack_07() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop(), AbstractStack::new(1..1,vec![]));
}

#[test]
fn test_abstract_stack_08() {
    let st = AbstractStack::new(0..0,vec![]);
    assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop().pop(), EMPTY_STACK);
}

#[test]
fn test_abstract_stack_09() {
    let st = AbstractStack::new(0..0,vec![ONE,TWO,THREE]);
    assert_eq!(st.values(),vec![ONE,TWO,THREE]);
    assert_eq!(st.set(0,ZERO),AbstractStack::new(0..0,vec![ONE,TWO,ZERO]));
}

#[test]
fn test_abstract_stack_0a() {
    let st = AbstractStack::new(0..0,vec![ONE,TWO,THREE]);
    assert_eq!(st.set(1,ZERO),AbstractStack::new(0..0,vec![ONE,ZERO,THREE]));
}

#[test]
fn test_abstract_stack_0b() {
    let st = AbstractStack::new(0..0,vec![ONE,TWO,THREE]);
    assert_eq!(st.set(2,UNKNOWN),AbstractStack::new(1..1,vec![TWO,THREE]));
}

#[test]
fn test_abstract_stack_0c() {
    let st = AbstractStack::new(0..0,vec![ONE,UNKNOWN,THREE]);
    assert_eq!(st.values(),vec![ONE,UNKNOWN,THREE]);
    assert_eq!(st.set(2,UNKNOWN),AbstractStack::new(2..2,vec![THREE]));
}

#[test]
fn test_abstract_stack_0d() {
    let st = AbstractStack::new(1..1,vec![]);
    assert_eq!(st.set(0,ONE),AbstractStack::new(0..0,vec![ONE]));
}

#[test]
fn test_abstract_stack_0e() {
    let st = AbstractStack::new(1..1,vec![TWO]);
    assert_eq!(st.set(1,ONE),AbstractStack::new(0..0,vec![ONE,TWO]));
}

#[test]
fn test_abstract_stack_0f() {
    let st = AbstractStack::new(2..2,vec![TWO]);
    assert_eq!(st.set(2,ONE),AbstractStack::new(0..0,vec![ONE,UNKNOWN,TWO]));
}

#[test]
fn test_abstract_stack_0g() {
    let st = AbstractStack::new(2..3,vec![TWO]);
    assert_eq!(st.set(2,ONE),AbstractStack::new(0..1,vec![ONE,UNKNOWN,TWO]));
}

#[test]
fn test_abstract_stack_10() {
    let st1 = AbstractStack::new(1..1,vec![ONE]);
    let st2 = AbstractStack::new(0..0,vec![ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(0..1,vec![ONE]));
}
#[test]
fn test_abstract_stack_11() {
    let st1 = AbstractStack::new(0..0,vec![ONE]);
    let st2 = AbstractStack::new(1..1,vec![ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(0..1,vec![ONE]));
}
#[test]
fn test_abstract_stack_12() {
    let st1 = AbstractStack::new(0..0,vec![TWO,ONE]);
    let st2 = AbstractStack::new(1..1,vec![ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(1..1,vec![ONE]));
}
#[test]
fn test_abstract_stack_13() {
    let st1 = AbstractStack::new(1..1,vec![ONE]);
    let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(1..1,vec![ONE]));
}
#[test]
fn test_abstract_stack_14() {
    let st1 = AbstractStack::new(0..0,vec![ONE,ONE]);
    let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(1..1,vec![ONE]));
}
#[test]
fn test_abstract_stack_15() {
    let st1 = AbstractStack::new(1..1,vec![ONE,ONE]);
    let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(1..2,vec![ONE]));
}
#[test]
fn test_abstract_stack_16() {
    let st1 = AbstractStack::new(0..1,vec![ONE,ONE]);
    let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(1..2,vec![ONE]));
}
#[test]
fn test_abstract_stack_17() {
    let st1 = AbstractStack::new(0..0,vec![ONE,ONE]);
    let st2 = AbstractStack::new(0..1,vec![TWO,ONE]);
    assert_eq!(st1.merge(&st2),AbstractStack::new(1..2,vec![ONE]));
}

// Tests for set()
// Force upper expansion
// Force rebalance
