(module
	(import "seal0" "seal_debug_message" (func $seal_debug_message (param i32 i32) (result i32)))
	(import "env" "memory" (memory 1 1))

	(data (i32.const 0) "Hello World!")

	(func $assert_eq (param i32 i32)
		(block $ok
			(br_if $ok
				(i32.eq (local.get 0) (local.get 1))
			)
			(unreachable)
		)
	)

	(func (export "call")
		(call $assert_eq
			(call $seal_debug_message
				(i32.const 0)	;; Pointer to the text buffer
				(i32.const 12)	;; The size of the buffer
			)
			(i32.const 0)	;; Success return code
		)
	)

	(func (export "deploy"))
)
