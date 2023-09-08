(module
(func $add (result i64)
i64.const 1
i64.const 2
i64.add
i64.const 3
i64.add
return
)
(export "add" (func $add))
)
