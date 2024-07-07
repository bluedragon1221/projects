(defmacro infix (x op y) `(,op ,x ,y))

(format t "~A~%" (infix 2 + (infix 5 - 3)))
