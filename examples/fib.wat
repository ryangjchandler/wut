(module
(func $fib (param $n i64) (result i64)
local.get $n
i64.const 2
i64.lt_u
(if
(then
local.get $n
return
)
)
local.get $n
i64.const 1
i64.sub
call $fib
local.get $n
i64.const 2
i64.sub
call $fib
i64.add
return
)
(export "fib" (func $fib))
)
