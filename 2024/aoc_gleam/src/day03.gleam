import gleam/int
import gleam/io
import gleam/list
import gleam/regexp
import gleam/result
import gleam/string
import simplifile

pub fn day03() {
  io.println("===== Day 03 =====")

  let filepath = "./input/day03/part1"
  let assert Ok(file) = simplifile.read(from: filepath)

  io.print("part 1: ")
  part1(file)
  io.print("part 2: ")
  part2(file)
}

fn part2(file) {
  let assert Ok(re) =
    regexp.from_string("mul\\([0-9]+,[0-9]+\\)|do\\(\\)|don't\\(\\)")

  regexp.scan(re, file)
  |> list.map(content)
  |> parse_enabled
  |> list.flat_map(extract)
  |> list.map(multiply)
  |> add
  |> io.debug
}

type State {
  CurrentState(current_keyword: String, instructions: List(String))
}

fn parse_enabled(l) {
  let initial_state = CurrentState("do()", list.new())
  let state = list.fold(l, initial_state, extract_instruction)
  state.instructions
}

fn extract_instruction(state: State, instruction: String) {
  case instruction {
    "do()" ->
      CurrentState(
        current_keyword: instruction,
        instructions: state.instructions,
      )
    "don't()" ->
      CurrentState(
        current_keyword: instruction,
        instructions: state.instructions,
      )
    _ ->
      case state.current_keyword {
        "do()" ->
          CurrentState(
            current_keyword: state.current_keyword,
            instructions: list.append(
              state.instructions,
              list.wrap(instruction),
            ),
          )
        _ -> state
      }
  }
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
