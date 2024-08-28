(defmacro $ (x op y) `(,op ,x ,y))

(format t "~A~%" ($ 2 + ($ 5 - 3)))
