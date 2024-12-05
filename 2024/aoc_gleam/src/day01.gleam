import gleam/dict
import gleam/int
import gleam/io
import gleam/list.{map}
import gleam/pair
import gleam/result
import gleam/string.{split}
import simplifile.{read}

pub fn day01() {
  io.println("===== Day 01 =====")

  part1()
  part2()
}

pub fn part2() {
  let filepath = "./input/day01/part1"
  let assert Ok(file) = read(from: filepath)
  let #(first, second) =
    split(file, on: "\n")
    |> list.flat_map(fn(line) {
      split(line, on: "   ")
      |> map(int.parse)
      |> map(fn(x) { result.unwrap(x, 0) })
      |> list.window_by_2
    })
    |> list.unzip

  let counts =
    list.fold(first, dict.new(), fn(a, x) {
      dict.insert(a, x, list.count(second, fn(y) { x == y }))
    })

  let _ =
    first
    |> list.map(fn(y) { int.multiply(y, result.unwrap(dict.get(counts, y), 0)) })
    |> fn(x) { list.fold(x, 0, int.add) }
    |> io.debug
}

fn part1() {
  let filepath = "./input/day01/part1"
  let assert Ok(file) = read(from: filepath)
  let _ =
    split(file, on: "\n")
    |> list.flat_map(fn(line) {
      split(line, on: "   ")
      |> map(int.parse)
      |> map(fn(x) { result.unwrap(x, 0) })
      |> list.window_by_2
    })
    |> list.unzip
    |> pair.map_first(fn(x) { list.sort(x, by: int.compare) })
    |> pair.map_second(fn(x) { list.sort(x, by: int.compare) })
    |> fn(x) { list.zip(pair.first(x), pair.second(x)) }
    |> map(fn(x) {
      int.subtract(pair.first(x), pair.second(x)) |> int.absolute_value
    })
    |> fn(x) { list.fold(x, 0, int.add) }
    |> io.debug
}
