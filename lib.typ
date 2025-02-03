#let wasm_plugin = plugin("./rust.wasm")

#let print(m, truncate: (0, 0)) = {
  assert((
    truncate.at(0) >= 0
    and truncate.at(1) >= 0
    and truncate.at(0) != 1
    and truncate.at(1) != 1
    and truncate.at(0) < m.at("matrix").at(1)
    and truncate.at(1) < m.at("matrix").at(2)
  ), message: "Invalid truncate parameter")
  let command = json.decode(str(wasm_plugin.to_mat(bytes(json.encode((..m, truncate: truncate)))))).command
  eval(command, mode: "math")
}
#let p = print

#let create(m) = {
  let wasm_result

  if type(m) == array {
    if type(m.at(0)) == int {
      wasm_result = wasm_plugin.create_matrix(bytes(json.encode((x: m.len(), y: 1, vec: m.flatten()))))
    } else if type(m.at(0)) == array {
      wasm_result = wasm_plugin.create_matrix(bytes(json.encode((x: m.len(), y: m.at(0).len(), vec: m.flatten()))))
    }
  } else {
    let type = type(m)
    assert(false, message: "Invalid input")
  }
  let json_result = json.decode(wasm_result)
  (
    p: print(json_result),
    ..json_result
  )
}

// * ---------- Matrix operations
#let add(..m) = {
  let wasm_result = wasm_plugin.add(bytes(json.encode((matrices: m.pos().map(x => x.matrix)))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}

#let sub(..m) = {
  let wasm_result = wasm_plugin.subtract(bytes(json.encode((matrices: m.pos().map(x => x.matrix)))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let subtract = sub

#let mul(..m) = {
  let wasm_result = wasm_plugin.multiply(bytes(json.encode((matrices: m.pos().map(x => x.matrix)))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let multiply = mul
#let product = mul

#let modify(m, location, value) = {
  let wasm_result = wasm_plugin.modify(bytes(json.encode((matrix: m.matrix, location: location, value: value))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
