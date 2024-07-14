---
title: I Probably Hate Writing Code in Your Favorite Language
slug: hate-your-language
author: Ashton Wiersdorf (lambdaland.org)
tags: ["Functional Programming"]
---

> The Tao gave birth to machine language. Machine language gave birth to the assembler.
> 
> The assembler gave birth to the compiler. Now there are ten thousand languages.
> 
> Each language has its purpose, however humble. Each language expresses the Yin and Yang of software. Each language has its place within the Tao.
> 
> But do not program in COBOL if you can avoid it.

I probably hate writing code in your favorite programming language, whatever it may be. This is because I get frustrated by basically all of the top 10 languages you’ll find listed anywhere for various reasons. Do I hate programming in Python? You bet I do. “But it’s _Python_! Python is the best programming language on earth!” I can hear you say. I grant that it has its place. Python wins because its ecosystem of libraries is so huge and because there are so many resources for new users. It’s also garbage collected, which means memory safety is not an issue. It the current hot thing, because there is so much support for machine learning in Python.

But _my_ problem is that Python is a boring language. This isn’t a bad thing necessarily. If you’re interested in solving a problem with a known solution and you’re doing it for business, the a boring language is probably better for you than, say, Haskell.

Why do I think Python is boring? In part because of its philosophy:

> There should be one—and preferably only one—obvious way to do it.
> 
> [The Zen of Python](https://peps.python.org/pep-0020/)

Python has a model of how it wants you to solve problems. That’s right: it wants you to solve problems with objects, classes, and explicit loops. Got a problem that’s the perfect fit for a functional paradigm? Well, I guess you can use `map` and `filter`, but you only get a single expression inside of lambdas, data structures are all mutable, and you can’t use recursion to handle lists. Ugh.

I could tell similar stories for other languages that I don’t like programming in. These languages include JavaScript, Go, Java, and C++. Go and Java seem to have been made with huge teams of programmers in mind: make the language and syntax as simple as possible, and then even simpler at the expense of expressivity! This guards against programmers coming up with a clever way to express their problem in a domain-specific way—that’s probably a virtue in large companies. But that’s not how I like to program.

## No local reasoning

The thing I hate about _all_ of the languages I listed is their emphasis on mutation. When I call a function and pass it a list or object or whatever, I have _no_ guarantees about that thing’s value when the function returns. That means, to understand some code, I have to understand all of the functions that get called.

In contrast, when I write in a language like Elixir or Haskell, which have _immutable_ data structures, I can look at some code like this:

```haskell
winningTeam :: GameLog -> Team winningTeam g = let (team_a, team_b) = getTeams g points_a = getPoints g team_a points_b = getPoints g team_b in if points_a > points_b then team_a else team_b
```

and I don’t have to know what `getTeams` or `getPoints` do to their arguments; I just know they return a value of some kind; I’m free to continue using `g`, `team_a`, and `team_b` as much as I like because their value has not changed.

It might not seem like much in this example, but it is a big deal when you’re neck-deep in a debugging session. I once worked on a codebase that was half in Elixir and half in Ruby. I spent most of my time on the Elixir side. One time when I had to do some debugging in Ruby, I found it so difficult to trace the execution of the program because data was being changed in method calls. If this doesn’t make much sense to you, you might have to experience it first: once you’ve worked in a large functional codebase, you will find yourself bewildered by all the spooky-action-at-a-distance that goes on inside a large OO codebase.

## Other gripes

Other things that frustrate me in programming languages include:

-   Automatic type conversion (looking at you JavaScript).
-   No type inference (if you’re gonna be statically typed, don’t make me write out the type every time Java).
-   No structural typing (type is determined by the shape, not the class name).
-   No good functional data structures.
-   No metaprogramming.
-   No TCO/limits on stack depth.

That last one is something that _really_ bothers me about Python: stack frames in Racket cost 2 words. Source: I asked Matthew Flatt about Racket’s stack frame size once. Either do proper tail-call elimination _or_, if you _really absolutely must_ have _all_ of your precious stack frames _performance and elegance be darned_, then _allocate your stack frames on the heap and stop worrying about it already!_ (I seem to recall a conversation where someone with knowledge of these things implied that this was in the works. I don’t know any details about it though.)

Seriously though: some solutions lend themselves _really_ well to a nice tail-recursive solution. But can you rely on such an implementation to be performant or even _run_ in Python? Nope. Argh!!

## My favorite language

Clearly, I like functional programming: it fits how my mind works, and I think it is in a lot of ways objectively better than other paradigms for software engineering. Immutability gives you the ability to reason _locally_ about your code—not to mention not having to worry about race conditions when mutating data in concurrent environments! To parallel the first list that I wrote, here are things that I like in a language:

-   Easy, explicit conversions between different types of data.
-   Dynamic typing _or_ powerful type inference _or_ gradual typing!
-   Structural typing (having nominal typing too can be nice when needed; but given one or the other I’ll take structural over nominal any day).
-   Functional data structures like cons cells, maps and sets supporting functional updates, and [RRB trees](https://github.com/OxSon/rrb.rhm/discussions/1)!
-   Powerful macros that let me extend the language.
-   Proper TCO.

Macros can be a two-edged sword. That said, a lot of the [danger around macros](https://lambdaland.org/posts/2023-10-17_fearless_macros/) has largely been ameliorated. Elixir is a great example of this: Elixir has a small core and uses macros a _lot_ to define basic things like `if` in terms of simpler constructs.

What languages do I _enjoy_ programming in? Racket is my favorite: it’s designed to be flexible and give the programmer maximum ability to express their intent to the computer. Racket is a _programmable programming language_.

Other languages I enjoy include Haskell, Elixir, and Rust. Haskell is the ur-functional language, and it’s really fun to use the type system to describe your domain. Pretty soon the compiler starts keeping you from making all sorts of mistakes that would be hard to catch with basic testing. In Elixir, you get lots of nice functional data structures, proper TCO, pattern matching, and soon gradual typing! Rust is great because it has a phenomenal type system with good type inference; its metaprogramming story could be improved though.

## Not a flame war

I want to make it clear that I am _not_ attempting to start a flame-war or saying that Python, Java, et al. are useless: they have their place and are very respectable works of engineering. All I am saying is that, given a choice of language for a hobby project, I will pick something else because I don’t want to be frustrated by the language when I work.

Anyway, that’s the end of my griping about languages. (For today, at least.)

> There will always be things we wish to say in our programs that in all known languages can only be said poorly.
> 
> A language that doesn’t affect the way you think about programming, is not worth knowing.