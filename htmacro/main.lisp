#!/usr/bin/env sbcl --script
(require "asdf")

(asdf:load-system "str")

(asdf:load-system "arrow-macros")
(use-package :arrow-macros)

; function that asks, "is `c` the first character?"
(defun first-char-p (c) (lambda (x) (-> x str:s-first (string= c))))

; "" -> nil
(defun or-nil (v) (if (> (length v) 0) v nil))

(defun comp (c components)
  (-<>> components
    str:downcase
    (str:replace-using (list "." " ." "#" " #"))
    (str:split " ")
    (remove "" <> :test #'string=)
    (remove-if-not (first-char-p c))
    (str:join " ")
    (str:replace-all c "")
    or-nil))

(defun fmt-class-id (inp)
  (format nil
      "~@[class='~A'~]~@[ id='~A'~]"
      (comp "." inp)
      (comp "#" inp)))

(defmacro h1 (first-arg &optional second-arg)
  (if `,second-arg
    (format nil
      "<h1 ~A>~A</h1>"
      (fmt-class-id (string `,first-arg))
      `,second-arg)

      (format nil "<h1>~A</h1>" `,first-arg)))

(defun stdout (&rest args)
  (dolist (<> args) (format t "~A~%" <>)))

(stdout (h1 .header.bg-blue#id.another-class "Hello")
        (h1 "Some other stuff"))

(quit) 
