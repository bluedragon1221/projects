(ql:quickload "str")

(defstruct range low high)
(defun range-as-pair (range) (list (range-low range) (range-high range)))

(defun fmt-range (range)
  (format nil "~A" (range-as-pair range)))

(defun upd-range (range guess-grade)
  (format t "~A~%" guess-grade)
  (let ((prev-guess (avg-range range)))
    (str:string-case guess-grade
      ("higher" (setf (range-low range) prev-guess))
      ("lower" (setf (range-high range) prev-guess))
      ("correct" nil)
      (otherwise nil))))

; ash halfs a number and makes it an int
(defun avg-range (range)
  (ash (reduce '+ (range-as-pair range)) -1))

(defun report-range (range)
  (format t "Range: ~A~%" (fmt-range range))
  (format t "Guess: ~A~%" (avg-range range)))

(let ((range (make-range :low 0 :high 100)))
  (upd-range range "lower")
  (report-range range))
