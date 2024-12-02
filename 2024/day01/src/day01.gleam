import gleam/int
import gleam/io
import gleam/list.{map}
import gleam/pair
import gleam/result
import gleam/string.{split}
import simplifile.{read}

pub fn main() {
  part1()
}

fn part1() {
  let filepath = "./input/part1"
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
