(ql:quickload '(:str :arrow-macros :fn) :silent t)
(named-readtables:in-readtable :fn.reader)
(use-package :arrow-macros)

(defun first-char-p (c) λ(-> _ str:s-first (char= c)))
(defun or-nil (v) (-> v length (> 0) (if v nil)))
(defun comp (char components)
  (->> components
    (remove-if-not (first-char-p char))
    (str:join " ")
    (str:replace-all char "")
    or-nil))

(defmacro h1 (first-arg &optional second-arg)
  (if `,second-arg
      (-<>> `,first-arg
        str:downcase
        (str:replace-using (list "." " ." "#" " #"))
        (str:split " ")
        (-<> <>
          (format nil
            "<h1~@[ class='~A'~]~@[ id='~A'~]>~A</h1>"
            (comp "." <>)
            (comp "#" <>)
            `,second-arg)))
      
      (format nil "<h1>~A</h1>" `,first-arg)))

(defun stdout (&rest args)
  (dolist (<> args) (format t "~A~%" <>)))

(stdout (h1 .header.bg-blue#id "Hello")
        (h1 "Some other stuff"))

(quit) 
