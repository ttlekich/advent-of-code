import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let filepath = "./input/part1"
  let assert Ok(file) = simplifile.read(from: filepath)

  part1(file)
  part2(file)
}

fn part2(file) {
  parse(file)
  |> list.map(combinations)
  |> list.map(diff2)
  |> list.map(check2)
  |> count
  |> io.debug
}

fn part1(file) {
  parse(file)
  |> list.map(diff)
  |> list.map(check)
  |> count
  |> io.debug
}

fn parse(file) {
  string.split(file, on: "\n")
  |> list.map(fn(line) {
    string.split(line, on: " ")
    |> list.map(fn(s) { result.unwrap(int.parse(s), 0) })
  })
}

fn diff(l) {
  l |> list.window_by_2 |> list.map(fn(p) { pair.first(p) - pair.second(p) })
}

fn diff2(ls) {
  ls |> list.map(diff)
}

fn combinations(l: List(Int)) {
  let combinations = list.combinations(l, list.length(l) - 1)
  list.append(combinations, list.wrap(l))
}

fn check(l) {
  { list.all(l, is_positive) || list.all(l, is_negative) }
  && !list.any(l, is_zero)
  && !list.any(l, is_greater_than_abs(3))
}

fn check2(ls) {
  ls |> list.map(check) |> fn(x) { list.any(x, fn(y) { y }) }
}

fn is_positive(a) {
  a > 0
}

fn is_negative(a) {
  a < 0
}

fn is_zero(a) {
  a == 0
}

fn is_greater_than_abs(n) {
  fn(x) { int.absolute_value(x) > n }
}

fn count(l) {
  list.fold(l, 0, fn(a, i) {
    a
    + case i {
      False -> 0
      True -> 1
    }
  })
}
