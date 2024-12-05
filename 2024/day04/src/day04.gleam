import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/string
import simplifile

pub fn main() {
  let filepath1 = "./input/part1"
  let assert Ok(file) = simplifile.read(from: filepath1)
  io.print("part 1: ")
  part1(file)

  let filepath2 = "./input/part1"
  let assert Ok(file) = simplifile.read(from: filepath2)
  io.print("part 2: ")
  part2(file)
}

fn part2(file) {
  let lines =
    file
    |> to_array_array

  let assert Ok(first) = list.first(lines)
  let n_columns = list.length(first)
  let n_rows = list.length(lines)
  let xs = list.range(from: 0, to: n_columns)
  let ys = list.range(from: 0, to: n_rows)
  let matrix = to_matrix(lines)

  list.flat_map(ys, fn(y) { list.map(xs, fn(x) { check2(matrix, x, y) }) })
  |> count
  |> io.debug
}

fn part1(file) {
  let lines = file |> to_array_array
  let assert Ok(first) = list.first(lines)
  let n_columns = list.length(first)
  let n_rows = list.length(lines)
  let xs = list.range(from: 0, to: n_columns)
  let ys = list.range(from: 0, to: n_rows)

  let matrix = to_matrix(lines)

  list.flat_map(ys, fn(y) { list.map(xs, fn(x) { check(matrix, x, y) }) })
  |> count
  |> io.debug
}

fn check(matrix, x, y) {
  list.map(directions, fn(d) { check_direction(matrix, x, y, d, chars) })
  |> count
}

fn check2(matrix, x, y) {
  let current_char = dict.get(matrix, #(x, y))
  case current_char == Ok("A") {
    True -> {
      check_for_x(matrix, x, y)
    }
    _ -> 0
  }
}

fn check_for_x(matrix, x, y) {
  let west_chars =
    [SouthWest, NorthWest]
    |> list.map(fn(d) { get_char(matrix, x, y, d) })
  let east_chars =
    [SouthEast, NorthEast]
    |> list.map(fn(d) { get_char(matrix, x, y, d) })
  case west_chars {
    [Ok("M"), Ok("M")] -> {
      case east_chars {
        [Ok("S"), Ok("S")] -> 1
        _ -> 0
      }
    }
    [Ok("S"), Ok("S")] -> {
      case east_chars {
        [Ok("M"), Ok("M")] -> 1
        _ -> 0
      }
    }
    [Ok("S"), Ok("M")] -> {
      case east_chars {
        [Ok("S"), Ok("M")] -> 1
        _ -> 0
      }
    }
    [Ok("M"), Ok("S")] -> {
      case east_chars {
        [Ok("M"), Ok("S")] -> 1
        _ -> 0
      }
    }
    _ -> 0
  }
}

fn get_char(matrix, x, y, d) {
  let direction_pair = direction_to_pair(d)
  let new_x = int.add(pair.first(direction_pair), x)
  let new_y = int.add(pair.second(direction_pair), y)
  dict.get(matrix, #(new_x, new_y))
}

fn check_direction(matrix, x, y, direction, chars) {
  let expected_char = list.first(chars)
  let current_char = dict.get(matrix, #(x, y))
  let rest = list.rest(chars)
  case rest {
    Ok(rest) -> {
      case Ok(expected_char) == Ok(current_char) {
        True -> {
          let direction_pair = direction_to_pair(direction)
          let new_x = int.add(pair.first(direction_pair), x)
          let new_y = int.add(pair.second(direction_pair), y)
          check_direction(matrix, new_x, new_y, direction, rest)
        }
        _ -> 0
      }
    }
    _ -> 1
  }
}

type Direction {
  North
  NorthEast
  East
  SouthEast
  South
  SouthWest
  West
  NorthWest
}

const directions = [
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
]

const chars = ["X", "M", "A", "S"]

fn direction_to_pair(d: Direction) {
  case d {
    North -> #(0, -1)
    NorthEast -> #(1, -1)
    East -> #(1, 0)
    SouthEast -> #(1, 1)
    South -> #(0, 1)
    SouthWest -> #(-1, 1)
    West -> #(-1, 0)
    NorthWest -> #(-1, -1)
  }
}

fn to_array_array(file) {
  string.split(file, on: "\n")
  |> list.map(fn(line) { string.split(line, on: "") })
}

fn to_matrix(lines) {
  let state =
    list.fold(lines, State(matrix: dict.new(), x: 0, y: 0), set_indexes)
  state.matrix
}

pub type State {
  State(matrix: dict.Dict(#(Int, Int), String), x: Int, y: Int)
}

fn set_indexes(state: State, line: List(String)) {
  let state = list.fold(line, state, traverse_x)
  let y = state.y
  State(matrix: state.matrix, x: 0, y: y + 1)
}

fn traverse_x(state: State, char: String) {
  let x = state.x
  let y = state.y
  let matrix = dict.insert(state.matrix, #(x, y), char)
  State(matrix: matrix, x: x + 1, y: y)
}

fn count(l) {
  l |> fn(l) { list.fold(l, 0, int.add) }
}
