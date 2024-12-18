use rand::prelude::*;
use std::{thread, time};

const INITIAL_POINTS:             usize = 1000;
const MAX_ITERATIONS:             usize = 100000;

const MIN_DISTANCE_X:             f64 = -50.0;
const MAX_DISTANCE_X:             f64 = 50.0;

const MIN_DISTANCE_Y:             f64 = 70.0;
const MAX_DISTANCE_Y:             f64 = 80.0;

const SIZE_OF_HORIZON_OF_EVENTS:  f64 = 0.001; // 0.1%

const HORIZON_OF_EVENTS_X:        f64 = (MAX_DISTANCE_X - MIN_DISTANCE_X) * SIZE_OF_HORIZON_OF_EVENTS;
const HORIZON_OF_EVENTS_Y:        f64 = (MAX_DISTANCE_Y - MIN_DISTANCE_Y) * SIZE_OF_HORIZON_OF_EVENTS;

const PUNISH_SCORE:       f64 = f64::MAX;

pub fn polynomial_text() -> String {
  return "x^7 + y^7 - 7 * (x^5 * y^2) + 14 * (x^3 * y^4) - 7 * (x * y^6) + 1500".to_string();
}

pub fn x_range_text() -> String {
  return "-50 < X < 50".to_string();
}

pub fn y_range_text() -> String {
  return "70 < Y < 80".to_string();
}

pub fn search_optimal() {
  let mut galaxy_vector: Vec<Point> = create_initial_vector();

  let mut black_hole_index: usize = search_optimal_point_index_from_vector(&galaxy_vector);

  for i in 0..MAX_ITERATIONS {
    //let black_hole: &Point = &galaxy_vector[black_hole_index];

    println!("Iteration: {} of {}", i, MAX_ITERATIONS);
    println!("x: {}, y: {}, score: {}", galaxy_vector[black_hole_index].x, galaxy_vector[black_hole_index].y, galaxy_vector[black_hole_index].score);
    println!("black hole position x: {}, y: {}", galaxy_vector[black_hole_index].x, galaxy_vector[black_hole_index].y);

    for j in 0..galaxy_vector.len() {
      // skip the actual black hole
      if black_hole_index == j { continue }

      // if score is equal to punishment score, then the point is eliminated and replace by another point in a valid range
      if galaxy_vector[j].score == PUNISH_SCORE {
        galaxy_vector[j] = create_random_point();
      }

      // if the actual point have a minus score of the black hole, then the points is the new black hole, skip iterartion
      if galaxy_vector[black_hole_index].score.abs() > galaxy_vector[j].score.abs() { 
        black_hole_index = j;
        println!("new black hole position x: {}, y: {}, score: {}", galaxy_vector[black_hole_index].x, galaxy_vector[black_hole_index].y, galaxy_vector[black_hole_index].score);
        //thread::sleep(time::Duration::from_millis(5000));
        //panic!("");
        continue; 
      }

      // if the point is inside in the horizon of events, must be replace for a new one
      //println!("distance {} a {}", HORIZON_OF_EVENTS, calculate_distance_between_points(&galaxy_vector[black_hole_index], &galaxy_vector[j]));
      if HORIZON_OF_EVENTS_X > calculate_distance_between_points(&galaxy_vector[black_hole_index], &galaxy_vector[j])
        && HORIZON_OF_EVENTS_Y > calculate_distance_between_points(&galaxy_vector[black_hole_index], &galaxy_vector[j]) {
        //println!("create new point");
        galaxy_vector[j] = create_random_point();
      } else { 
        // move the point to the black hole
        let point_in_new_position: Point = create_new_point_to_back_hole_by_index(&galaxy_vector, black_hole_index, j);
        //println!("move point in cordinates x: {}, y: {} to x: {}, y:{}", galaxy_vector[j].x, galaxy_vector[j].y, point_in_new_position.x, point_in_new_position.y);
        galaxy_vector[j] = point_in_new_position;
      }
    }
  }

  println!();
  println!("Final result: x: {}, y: {}, score: {}", galaxy_vector[black_hole_index].x, galaxy_vector[black_hole_index].y, galaxy_vector[black_hole_index].score);
  println!("Rules {} and {}", x_range_text(), y_range_text());
}

// private

struct Point {
  x: f64,
  y: f64,
  score: f64
}

impl Point {
  pub fn new(x: f64, y: f64) -> Self {
    Point {
          x: x,
          y: y,
          score: evaluate_polynomial_by_cordinates(x, y)//.abs()
      }
  }
}

fn create_new_point_to_back_hole_by_index(galaxy_vector: &Vec<Point>, black_hole_index: usize, point_index: usize) -> Point {
  let distance: f64 = calculate_distance_between_points(&galaxy_vector[black_hole_index], &galaxy_vector[point_index]);

  let movement: f64 = distance * generate_random_number(0.0, 1.0);

  let unit_vector_x: f64 = (galaxy_vector[point_index].x - galaxy_vector[black_hole_index].x) / distance;
  let unit_vector_y: f64 = (galaxy_vector[point_index].y - galaxy_vector[black_hole_index].y) / distance;

  let new_x_position: f64 = galaxy_vector[black_hole_index].x - unit_vector_x * movement;
  let new_y_position: f64 = galaxy_vector[black_hole_index].y - unit_vector_y * movement;

  let new_point: Point = Point::new(new_x_position, new_y_position);

  return new_point;
}

fn points_are_equals(point_1: &Point, point_2: &Point) -> bool {
  point_1.x == point_2.x && point_1.y == point_2.y
}

fn calculate_distance_between_points(point_1: &Point, point_2: &Point) -> f64 {
  ((point_2.x - point_1.x).powi(2) + (point_2.y - point_1.y).powi(2)).sqrt()
}

fn evaluate_polynomial_by_point(point :Point) -> f64 {
  evaluate_polynomial_by_cordinates(point.x, point.y)
}

fn evaluate_polynomial_by_cordinates(x :f64, y :f64) -> f64  {
  let mut result: f64 = 
  x.powi(7) 
  + y.powi(7) 
  - 7.0 * (x.powi(5) * y.powi(2)) 
  + 14.0 * (x.powi(3) * y.powi(4)) 
  - 7.0 * (x * y.powi(6)) 
  + 1500.0;

  if result.is_nan() {
    println!("El valor calculado es NaN");
  }

  // punish system
  if !(50.0..5000.0).contains(&x) {
    //result = PUNISH_SCORE;
  }

  if !(400.0..450.0).contains(&y) {
    //result = PUNISH_SCORE;
  }

  return result;
}

fn create_initial_vector() -> Vec<Point> {
  let mut initial_vector: Vec<Point> = vec![];

  for i in 0..INITIAL_POINTS {
    let point: Point = create_random_point();

    initial_vector.push(point);
  }

  return initial_vector;
}

fn create_random_point() -> Point {
  let point: Point = Point::new(
    generate_random_number(MIN_DISTANCE_X, MAX_DISTANCE_X), 
    generate_random_number(MIN_DISTANCE_Y, MAX_DISTANCE_Y)
  );

  return point;
}

fn search_optimal_point_index_from_vector(points_vector :&[Point]) -> usize {
  let mut optimal_index: usize = 0;
  let mut optimal = match points_vector.first() {
    Some(point) => point,
    None => {
        // Handle the case where points_vector is empty
        // For example, you could return an error or a default value
        panic!("points_vector is empty");
    }
  };

  for i in 0..points_vector.len() {
    let point: &Point = &points_vector[i];
    if optimal.score.abs() > point.score.abs() {
      optimal = point;
      optimal_index = i;
    }
  }

  return optimal_index;
}

fn generate_random_number(min: f64, max: f64) -> f64 {
  let mut rng = rand::thread_rng(); // Crea un generador de números aleatorios

  let random_number: f64 = rng.gen_range(min..max);
  return random_number
}