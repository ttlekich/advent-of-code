import gleam/dict
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() {
  let filepath = "./input/part1"
  let assert Ok(file) = simplifile.read(from: filepath)

  io.print("part 1: ")
  part1(file)
}

fn part1(file) {
  let lines = file |> to_array_array
  let assert Ok(first) = list.first(lines)
  let n_columns = list.length(first)
  let n_rows = list.length(lines)
  let matrix = to_matrix(lines)
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
