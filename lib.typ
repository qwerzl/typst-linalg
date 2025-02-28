#let wasm_plugin = plugin("./rust.wasm")

// * ---------- Utilities
#let print(m, truncate: (0, 0), precision: 4) = {
  assert((
    truncate.at(0) >= 0
    and truncate.at(1) >= 0
    and truncate.at(0) != 1
    and truncate.at(1) != 1
    and (((m.at("matrix").at(1)) == none) or (truncate.at(0) < m.at("matrix").at(1)))
    and truncate.at(1) < m.at("matrix").at(2)
  ), message: "Invalid truncate parameter")
  assert(precision >= -1, message: "Invalid precision")

  let command = json.decode(str(wasm_plugin.to_mat(bytes(json.encode((..m, truncate: truncate, precision: precision)))))).command
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

#let _create_from_nalgebra(m) = {
  let dict = (
    matrix: m
  )

  // none are returned by into_rows() or into_columns(), which is weird
  if (m.at(1) == none) {
    dict.at("matrix").at(1) = 1
  }
  if (m.at(2) == none) {
    dict.at("matrix").at(2) = 1
  }

  (
    p: print(dict),
    ..dict
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

#let subtract(..m) = {
  let wasm_result = wasm_plugin.subtract(bytes(json.encode((matrices: m.pos().map(x => x.matrix)))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let sub = subtract

#let matmul(..m) = {
  let wasm_result = wasm_plugin.multiply(bytes(json.encode((matrices: m.pos().map(x => x.matrix)))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}

#let multiply(..m) = {
  let wasm_result = wasm_plugin.component_multiply(bytes(json.encode((matrices: m.pos().map(x => x.matrix)))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let mul = multiply
#let component_multiply = multiply

#let divide(..m) = {
  let wasm_result = wasm_plugin.component_division(bytes(json.encode((matrices: m.pos().map(x => x.matrix)))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let div = divide
#let component_divide = divide

#let modify(m, location, value) = {
  let wasm_result = wasm_plugin.modify(bytes(json.encode((matrix: m.matrix, location: location, value: value))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}

#let transpose(m) = {
  let wasm_result = wasm_plugin.transpose(bytes(json.encode((matrix: m.matrix))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}

// * ---------- Matrix Scalar Operations
#let add_scalar(m, scalar) = {
  let wasm_result = wasm_plugin.add_scalar(bytes(json.encode((matrix: m.matrix, scalar: scalar))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}

#let sub_scalar(m, scalar) = {
  let wasm_result = wasm_plugin.sub_scalar(bytes(json.encode((matrix: m.matrix, scalar: scalar))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let subtract_scalar = sub_scalar

#let mul_scalar(m, scalar) = {
  let wasm_result = wasm_plugin.multiply_scalar(bytes(json.encode((matrix: m.matrix, scalar: scalar))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let multiply_scalar = mul_scalar

#let div_scalar(m, scalar) = {
  let wasm_result = wasm_plugin.divide_scalar(bytes(json.encode((matrix: m.matrix, scalar: scalar))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
#let divide_scalar = div_scalar

// * ---------- Matrix Views

#let into_rows(m) = {
  let wasm_result = wasm_plugin.into_rows(bytes(json.encode((matrix: m.matrix))))
  let decoded_json = json.decode(wasm_result)
  decoded_json.at("matrices").map(x => _create_from_nalgebra(x))
}

#let into_columns(m) = {
  let wasm_result = wasm_plugin.into_columns(bytes(json.encode((matrix: m.matrix))))
  let decoded_json = json.decode(wasm_result)
  decoded_json.at("matrices").map(x => _create_from_nalgebra(x))
}

// * ---------- Matrix Apply Functions
#let sigmoid(m) = {
  let wasm_result = wasm_plugin.apply_sigmoid(bytes(json.encode((matrix: m.matrix))))
  let decoded_json = json.decode(wasm_result)
  (
    p: print(decoded_json),
    ..decoded_json
  )
}
