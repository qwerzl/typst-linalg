#{
  let p = plugin("./rust.wasm")

  assert.eq(str(p.hello()), "Hello from wasm!!!")
  assert.eq(json.decode(str(p.create_matrix(bytes(json.encode((x: 2, y: 2, vec: (1, 2, 3, 4))))))).id, 1)
  assert.eq(str(p.to_mat(bytes(json.encode((id: 1))))), "hello*world")
  assert.eq(str(p.shuffle(bytes("s1"), bytes("s2"), bytes("s3"))), "s3-s1-s2")
  assert.eq(str(p.returns_ok()), "This is an `Ok`")
  // p.will_panic()  // Fails compilation
  // p.returns_err() // Fails compilation with an error message
}
