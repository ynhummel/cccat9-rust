use project::{calculate_ride, Segment};

#[test]
fn primeiro_dia_do_mes() {
    assert_eq!(
        calculate_ride(vec![Segment::new(10, 2021, 3, 1, 10, 0, 0)]),
        15.0
    );
}

#[test]
fn corrida_diurna_nao_domingo() {
    assert_eq!(
        calculate_ride(vec![Segment::new(10, 2021, 3, 2, 10, 0, 0)]),
        21.0
    );
}

#[test]
fn corrida_noturna() {
    assert_eq!(
        calculate_ride(vec![Segment::new(10, 2021, 3, 2, 23, 0, 0)]),
        39.0
    );
}

#[test]
fn corrida_diurna_domingo() {
    assert_eq!(
        calculate_ride(vec![Segment::new(10, 2021, 3, 7, 10, 0, 0)]),
        29.0
    );
}

#[test]
fn corrida_noturna_domingo() {
    assert_eq!(
        calculate_ride(vec![Segment::new(10, 2021, 3, 7, 23, 0, 0)]),
        50.0
    );
}

#[test]
fn corrida_com_valor_minimo() {
    assert_eq!(
        calculate_ride(vec![Segment::new(3, 2021, 3, 1, 10, 0, 0)]),
        10.0
    );
}
