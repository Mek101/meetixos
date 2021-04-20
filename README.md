# MeetiX OS - What is it?
MeetiX is a **NON** Unix-Like microkernel based operating system written in Rust developed by [Marco Cicognani](marco.cicognani@meetixos.org).

# Story
The MeetiX's story starts at the end of the 2014, when I caught by an extreme interest in the subject of operating systems,<br>
I began to inform myself about this argument. How they works? How they are structured, and...in 2014 is possible to write<br>
an entire operating system by yourself?<br>

## The Response
Well, **no**, but **YES**, if you have a lot of time to invest, a lot of stuffs to learn, and a real passion...like me.<br>

## Iterations
The project have traversed various iterations, embrional states, implementations, drastical architecture changes,<br>
changes in thinking; all of these because I'm the only developer; so what I learn, I discover and how, it is always<br>
ported to the OS. This is why, despite his age, the project is not finished yet (and will never happen!).

# Branches
Each branch of the git project represents one iteration of the project. Not all of them work, not all of them are definable<br>
as operating systems (like the first iterations, more like a freestanding shells), but if you read the code you will see how<br>
in the years my coding and thinking mode are changed.

## Current Iteration (master Branch)
This implementation, I hope, will be the last rewrite of the project

# Architecture
The kernel + the libapi crate provide an object oriented set of abstractions to interact with the operating system 