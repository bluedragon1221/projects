(defun make-adder (num)
  (lambda (x) (+ num x)))

(format t "~A~%" (funcall (make-adder 5) 2))
