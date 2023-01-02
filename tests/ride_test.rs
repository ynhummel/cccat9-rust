use chrono::prelude::*;
use project::ride::Ride;

#[test]
fn primeiro_dia_do_mes() {
    let mut ride = Ride::new();
    let datetime = Utc.with_ymd_and_hms(2021, 3, 1, 10, 0, 0).unwrap();
    ride.add_segment(10, datetime);
    assert_eq!(ride.calculate_ride(), 15.0);
}

#[test]
fn corrida_diurna_nao_domingo() {
    let mut ride = Ride::new();
    let datetime = Utc.with_ymd_and_hms(2021, 3, 2, 10, 0, 0).unwrap();
    ride.add_segment(10, datetime);
    assert_eq!(ride.calculate_ride(), 21.0);
}

#[test]
fn corrida_noturna() {
    let mut ride = Ride::new();
    let datetime = Utc.with_ymd_and_hms(2021, 3, 2, 23, 0, 0).unwrap();
    ride.add_segment(10, datetime);
    assert_eq!(ride.calculate_ride(), 39.0);
}

#[test]
fn corrida_diurna_domingo() {
    let mut ride = Ride::new();
    let datetime = Utc.with_ymd_and_hms(2021, 3, 7, 10, 0, 0).unwrap();
    ride.add_segment(10, datetime);
    assert_eq!(ride.calculate_ride(), 29.0);
}

#[test]
fn corrida_noturna_domingo() {
    let mut ride = Ride::new();
    let datetime = Utc.with_ymd_and_hms(2021, 3, 7, 23, 0, 0).unwrap();
    ride.add_segment(10, datetime);
    assert_eq!(ride.calculate_ride(), 50.0);
}

#[test]
fn corrida_com_valor_minimo() {
    let mut ride = Ride::new();
    let datetime = Utc.with_ymd_and_hms(2021, 3, 1, 10, 0, 0).unwrap();
    ride.add_segment(3, datetime);
    assert_eq!(ride.calculate_ride(), 10.0);
}
