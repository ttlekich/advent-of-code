import gleam/int
import gleam/io
import gleam/list
import gleam/regexp
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let filepath = "./input/part1"
  let assert Ok(file) = simplifile.read(from: filepath)

  part1(file)
}

fn part1(file) {
  let assert Ok(re) = regexp.from_string("mul\\([0-9]+,[0-9]+\\)")
  regexp.scan(re, file)
  |> list.map(content)
  |> list.flat_map(extract)
  |> list.map(multiply)
  |> add
  |> io.debug
}

fn content(match: regexp.Match) {
  match.content
}

fn extract(s) {
  let assert Ok(re) = regexp.from_string("[0-9]+,[0-9]+")
  regexp.scan(re, s)
  |> list.map(content)
  |> list.map(fn(l) {
    string.split(l, on: ",")
    |> list.map(int.parse)
    |> list.map(fn(x) { result.unwrap(x, 0) })
  })
}

fn multiply(l) {
  list.fold(l, 1, fn(a, x) { int.multiply(a, x) })
}

fn add(l) {
  list.fold(l, 0, fn(a, x) { int.add(a, x) })
}
